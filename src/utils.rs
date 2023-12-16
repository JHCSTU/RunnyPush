pub struct Utils {
    data: Vec<String>,
}

impl Utils {
    pub fn new(data: Vec<String>) -> Utils {
        Utils {
            data
        }
    }
    pub fn get_count(&self) -> i32 {
        let mut index = 0;
        for key in self.data.iter() {
            if key.contains("true") {
                index = index + 1;
            }
        }
        index
    }
    pub fn get_average_speed(&self) -> f32 {
        let mut index = self.get_count();
        let mut sum = 0.0;
        for key in self.data.iter() {
            if key.contains("true") {
                if let Ok(temp_speed) = key.split("-").collect::<Vec<&str>>()[2].replace(" m/s", "").parse::<f32>() {
                    sum = sum + temp_speed;
                } else {
                    index = index - 1;
                }
            }
        }
        sum / index as f32
    }
}