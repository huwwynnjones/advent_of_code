use std::{
    convert::TryInto,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Layer {
    data: Vec<Vec<u32>>,
}

impl Layer {
    fn new(data: &[Vec<u32>]) -> Layer {
        Layer {
            data: Vec::from(data),
        }
    }

    pub(crate) fn count_occurences_of(&self, nmb: u32) -> u32 {
        let flattened = self.data.iter().flatten().copied().collect::<Vec<u32>>();
        match flattened.iter().filter(|x| **x == nmb).count().try_into() {
            Ok(i) => i,
            Err(_) => 0,
        }
    }

    pub(crate) fn data_as_message(&self) -> String {
        let mut text = String::new();

        for row in &self.data {
            for nmb in row {
                match nmb {
                    0 => text.push_str(" "),
                    1 => text.push_str("*"),
                    _ => panic!("Should never happen"),
                }
            }
            text.push('\n');
        }
        text
    }
}

#[derive(Eq, PartialEq, Debug)]
enum PixelColour {
    Black,
    White,
    Transparent,
}

impl From<u32> for PixelColour {
    fn from(pixel_colour_number: u32) -> Self {
        match pixel_colour_number {
            0 => PixelColour::Black,
            1 => PixelColour::White,
            2 => PixelColour::Transparent,
            _ => panic!("Unknown pixel colour {}", pixel_colour_number),
        }
    }
}

impl From<&u32> for PixelColour {
    fn from(pixel_colour_number: &u32) -> Self {
        match pixel_colour_number {
            0 => PixelColour::Black,
            1 => PixelColour::White,
            2 => PixelColour::Transparent,
            _ => panic!("Unknown pixel colour {}", pixel_colour_number),
        }
    }
}

pub(crate) fn create_layers_from_image_data(data: &[u32], width: u32, height: u32) -> Vec<Layer> {
    let mut layers = Vec::new();
    let mut input_data = Vec::from(data);
    input_data.reverse();
    while !input_data.is_empty() {
        let mut layer_data = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                if let Some(nmb) = input_data.pop() {
                    row.push(nmb)
                }
            }
            layer_data.push(row)
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

pub(crate) fn create_final_image(layers: &[Layer]) -> Layer {
    let mut final_data: Vec<Vec<u32>> = Vec::new();

    for layer in layers {
        for (r_idx, row) in layer.data.iter().enumerate() {
            for (c_idx, current_pixel) in row.iter().enumerate() {
                let final_pixel = final_data.get(r_idx).and_then(|v| v.get(c_idx));
                match final_pixel {
                    Some(pixel) => match PixelColour::from(pixel) {
                        PixelColour::Transparent => final_data[r_idx][c_idx] = *current_pixel,
                        PixelColour::Black | PixelColour::White => (),
                    },
                    None => match final_data.get_mut(r_idx) {
                        Some(v) => v.push(*current_pixel),
                        None => final_data.push(vec![*current_pixel]),
                    },
                }
            }
        }
    }

    Layer { data: final_data }
}

pub(crate) fn load_image_data(file_name: &str) -> io::Result<Vec<u32>> {
    let program_input = File::open(file_name)?;
    let mut reader = BufReader::new(program_input);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let data: Vec<u32> = buf
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
        let layer = Layer::new(&vec![vec![1, 4, 3, 4, 1, 6]]);
        assert_eq!(layer.count_occurences_of(4), 2);
        assert_eq!(layer.count_occurences_of(1), 2);
        assert_eq!(layer.count_occurences_of(9), 0)
    }

    #[test]
    fn test_create_layers_from_image_data() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let layers = create_layers_from_image_data(&data, 3, 2);
        assert_eq!(layers.get(0).unwrap().data.len(), 2);
        assert_eq!(layers.get(0).unwrap().data.get(0).unwrap().len(), 3);
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

    #[test]
    fn test_create_final_image() {
        let mut data = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        let mut layers = create_layers_from_image_data(&data, 2, 2);
        let mut correct_text = "01\n10\n";
        let mut final_image = create_final_image(&layers);
        assert_eq!(final_image.data_as_string(), correct_text);

        data = vec![0, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        layers = create_layers_from_image_data(&data, 2, 2);
        correct_text = "01\n11\n";
        final_image = create_final_image(&layers);
        assert_eq!(final_image.data_as_string(), correct_text)
    }
}
