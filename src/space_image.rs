use std::{
    convert::TryInto,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Layer {
    data: Vec<u32>,
}

impl Layer {
    fn new(data: &[u32]) -> Layer {
        Layer {
            data: Vec::from(data),
        }
    }

    pub(crate) fn count_occurences_of(&self, nmb: u32) -> u32 {
        match self.data.iter().filter(|x| **x == nmb).count().try_into() {
            Ok(i) => i,
            Err(_) => 0,
        }
    }
}

pub(crate) fn create_layers_from_image_data(data: &[u32], width: u32, height: u32) -> Vec<Layer> {
    let mut layers = Vec::new();
    let mut input_data = Vec::from(data);
    input_data.reverse();
    while !input_data.is_empty() {
        let mut layer_data = Vec::new();
        for _ in 0..(width * height) {
            if let Some(nmb) = input_data.pop() {
                layer_data.push(nmb)
            }
        }
        layers.push(Layer::new(&layer_data))
    }
    layers
}

pub(crate) fn find_layer_with_lowest_nmb(layers: &[Layer], nmb: u32) -> Option<&Layer> {
    let mut lowest_count = u32::max_value();
    let mut lowest_layer = None;
    for layer in layers {
        let count = layer.count_occurences_of(nmb);
        if count == 0 {
            continue;
        } else if count < lowest_count {
            lowest_count = count;
            lowest_layer = Some(layer)
        }
    }
    lowest_layer
}

pub(crate) fn load_image_data(file_name: &str) -> io::Result<Vec<u32>> {
    let program_input = File::open(file_name)?;
    let mut reader = BufReader::new(program_input);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let data = buf
        .trim_end_matches('\n')
        .chars()
        .map(|ch| {
            ch.to_digit(10)
                .unwrap_or_else(|| panic!("all input shoudl be numbers: {}", ch))
        })
        .collect();
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurences_of() {
        let layer = Layer::new(&vec![1, 4, 3, 4, 1, 6]);
        assert_eq!(layer.count_occurences_of(4), 2);
        assert_eq!(layer.count_occurences_of(1), 2);
        assert_eq!(layer.count_occurences_of(9), 0)
    }

    #[test]
    fn test_create_layers_from_image_data() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let layers = create_layers_from_image_data(&data, 3, 2);
        assert_eq!(layers.get(0).unwrap().data.len(), 6);
        assert_eq!(layers.get(1).unwrap().data.len(), 6);
        assert_eq!(layers.len(), 2)
    }

    #[test]
    fn test_find_layer_with_lowest_nmb() {
        let data = vec![1, 5, 3, 4, 5, 6, 7, 5, 3, 4, 9, 2];
        let layers = create_layers_from_image_data(&data, 3, 2);
        assert_eq!(*find_layer_with_lowest_nmb(&layers, 5).unwrap(), layers[1])
    }

    #[test]
    fn test_load_image_data() {
        let correct_data = vec![2, 1, 2, 2, 2, 2, 2];
        assert_eq!(
            load_image_data("image_data_test.txt").unwrap(),
            correct_data
        );
    }
}
