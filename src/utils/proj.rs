const DTR: f64 = 3.141592653589793238 / 180.0;
const SCALE_LATITUDE_3410: f64 = 30.0 * DTR;
const TOTAL_SCALE_3410: f64 = 6371228.0;

pub fn proj_4326_to_3410(coord: &Vec<f64>) -> Vec<f64> {
  let mut lng = coord[0]; // x
  let mut lat = coord[1]; // y
  let scale_factor = SCALE_LATITUDE_3410.cos();

  lng = lng * DTR;
  lat = lat * DTR;
  lng = lng * scale_factor;
  lat = lat.sin() / scale_factor;
  lng = lng * TOTAL_SCALE_3410;
  lat = lat * TOTAL_SCALE_3410;

  vec![lng, lat]
}

#[cfg(test)]
mod test {

  #[test]
  pub fn proj_4326_to_3410() {
    assert_eq!(super::proj_4326_to_3410(&vec![0.0, 0.0]), vec![0.0, 0.0]);
    assert_eq!(
      super::proj_4326_to_3410(&vec![102.0, 2.0]),
      vec![9822709.901422562, 256750.72533117904]
    ); // from exportify vec![9822709.90142256, 256750.725331179]
    assert_eq!(
      super::proj_4326_to_3410(&vec![180.0, 86.0]),
      vec![17334193.943686873, 7338939.45955284]
    ); // from exportify vec![17334193.9436869, 7338939.45955284]
    assert_eq!(
      super::proj_4326_to_3410(&vec![90.0, 43.0]),
      vec![8667096.971843436, 5017366.729193342]
    ); // from exportify vec![8667096.97184344, 5017366.72919334]
  }
}
