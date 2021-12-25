use std::io::Read;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    // TODO this should be from iterator
    pub fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> Grid<T>
    where
        T: From<u8>,
    {
        let mut width = usize::MAX;
        let mut start_new_line = true;

        let mut data = Vec::with_capacity(bytes.size_hint().1.unwrap_or(0));
        for (i, byte) in bytes.skip_while(|&b| b == b'\n').enumerate() {
            if byte == b'\n' {
                if width == usize::MAX {
                    width = i;
                }
                debug_assert_eq!(data.len() % width, 0);
                if start_new_line {
                    // found \n\n
                    break;
                }
                start_new_line = true;
                continue;
            }

            start_new_line = false;
            data.push(byte.into());
        }
        let height = data.len() / width;

        Grid {
            data,
            width,
            height,
        }
    }

    pub fn read(reader: &mut impl Read) -> Self
    where
        T: From<u8>,
    {
        Self::from_bytes(&mut reader.bytes().map(|b| b.unwrap()))
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = x + y * self.width;
        (index < self.data.len()).then(|| &self.data[index])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = x + y * self.width;
        (index < self.data.len()).then(|| &mut self.data[index])
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn swap(&mut self, pos1: (usize, usize), pos2: (usize, usize)) {
        let index1 = pos1.0 + pos1.1 * self.width;
        let index2 = pos2.0 + pos2.1 * self.width;
        self.data.swap(index1, index2);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn indices(&self) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width;
        (0..self.width * self.height).map(move |index| (index % width, index / width))
    }

    pub fn values(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        let width = self.width;
        (0..self.width * self.height)
            .map(move |index| (index % width, index / width, &self.data[index]))
    }

    pub fn kernel_3x3_at(&self, x: usize, y: usize, default: T) -> Kernel<'_, T>
    where
        T: Copy,
    {
        Kernel::new(self, (x as isize, y as isize), default, 1)
    }

    pub fn kernel_3x3(
        &self,
        default: T,
    ) -> impl Iterator<Item = ((usize, usize), Kernel<'_, T>)> + '_
    where
        T: Copy,
    {
        self.indices().map(move |(x, y)| {
            (
                (x, y),
                Kernel::new(self, (x as isize, y as isize), default, 1),
            )
        })
    }

    pub fn kernel_ext_3x3(
        &self,
        default: T,
    ) -> impl Iterator<Item = ((usize, usize), Kernel<'_, T>)> + '_
    where
        T: Copy,
    {
        let border = 1;

        let width = self.width + border * 2;
        let height = self.height + border * 2;
        let indices = (0..width * height).map(move |index| (index % width, index / width));

        indices.map(move |(x, y)| {
            (
                (x, y),
                Kernel::new(
                    self,
                    (x as isize - border as isize, y as isize - border as isize),
                    default,
                    border,
                ),
            )
        })
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.data.len() {
            if i > 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.data[i])?;
        }
        Ok(())
    }
}

impl<T: Read> std::convert::From<T> for Grid<u8> {
    fn from(reader: T) -> Grid<u8> {
        let mut width = 0;
        let data: Vec<_> = reader
            .bytes()
            .enumerate()
            .filter_map(|(i, v)| {
                let v = v.unwrap();
                let relevant = (b'0'..=b'9').contains(&v);
                if width == 0 && !relevant {
                    width = i;
                }
                relevant.then(|| v - b'0')
            })
            .collect();
        let height = data.len() / width;

        Grid {
            data,
            width,
            height,
        }
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x + y * self.width]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x + y * self.width]
    }
}

pub struct Kernel<'a, T: Copy> {
    grid: &'a Grid<T>,
    position: (isize, isize),
    default: T,
    current: usize,
    size: usize,
    border: usize,
}

impl<'a, T: Copy> Kernel<'a, T> {
    fn new(grid: &'a Grid<T>, position: (isize, isize), default: T, border: usize) -> Self {
        let size = border * 2 + 1;
        Self {
            grid,
            position,
            default,
            current: 0,
            size,
            border,
        }
    }
}

impl<'a, T: Copy> Iterator for Kernel<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.size * self.size {
            return None;
        }
        let (dx, dy) = (self.current % self.size, self.current / self.size);
        let (x, y) = (
            self.position.0 + dx as isize - self.border as isize,
            self.position.1 + dy as isize - self.border as isize,
        );

        self.current += 1;

        if x < 0 || x >= self.grid.width as isize || y < 0 || y >= self.grid.height as isize {
            Some(self.default)
        } else {
            Some(self.grid[(x as usize, y as usize)])
        }
    }
}
