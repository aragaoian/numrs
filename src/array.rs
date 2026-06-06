pub struct Array<T> {
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T> Array<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Result<Self, String> {
        if rows == 0 {
            return Err("Field 'rows' cannot be empty!".to_string());
        } else if cols == 0 {
            return Err("Field 'cols' cannot be empty!".to_string());
        }

        if rows * cols != data.len() {
            return Err("Data does not match the declared shape!".to_string());
        }

        Ok(Self {
            shape: vec![rows, cols],
            data,
        })
    }

    pub fn shape(&self) -> Vec<usize> {
        self.shape.clone()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn ndim(&self) -> usize {
        todo!("Not implemented")
    }

    pub fn size(&self) -> usize {
        todo!("Not implemented")
    }

    pub fn reshape(&self) -> usize {
        todo!("Not implemented")
    }
}