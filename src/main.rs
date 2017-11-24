extern crate image;

use image::{GenericImage, Pixel, Luma};
use std::f64::consts;


fn innerprod<I, J>(subimage: &I, costmpl: &J) -> f64 
  where I: GenericImage<Pixel = Luma<u8>>,
        J: GenericImage<Pixel = Luma<f64>>
  {
  let mut result = 0.0_f64;
  for y in 0..7{
    for x in 0..7 {
      result += (
        subimage.get_pixel(x, y).to_luma().data[0] as f64 *
        costmpl.get_pixel(x, y).to_luma().data[0]);
    }
  }
  result
}

fn main() {
  let mut img = image::open("map_gray_crop.png").unwrap().to_luma();
  let (dimx, dimy) = img.dimensions();
  let mut dctcoef :image::ImageBuffer<Luma<u8>, _> = image::ImageBuffer::new(dimx, dimy);
  let (repx, repy) = (dimx/8, dimy/8);

  let mut costmpl : Vec<Vec<image::ImageBuffer<Luma<f64>, Vec<f64>>>> = Vec::new();

  for v in 0usize..8 {
    let mut costmpl_u : Vec<image::ImageBuffer<Luma<f64>, Vec<f64>>> = Vec::new();
    for u in 0usize..8 {
      costmpl_u.push( 
        image::ImageBuffer::from_fn(8, 8, |x, y| {
          image::Luma([
                      ((2.0_f64 * x as f64 + 1.0_f64) * u as f64 * consts::PI/16.0_f64).cos() *
                       ((2.0_f64 * y as f64 + 1.0_f64) * v as f64 * consts::PI/16.0_f64).cos()
                      ])
        }));

      /*
      let tr = image::ImageBuffer::from_fn(8, 8, |x, y| {
        image::Luma([(costmpl_u[u].get_pixel(x, y).to_luma().data[0] * 128_f64 + 127_f64) as u8])
      });
      tr.save(format!("costmpl{}{}.png", u, v));
      */
    }
    costmpl.push(costmpl_u);
  }

  let costmpl = costmpl;

  for blocky in 0..repy {
    for blockx in 0..repx{
      let subimage = img.sub_image(blockx*8, blocky*8, 8, 8);
      for v in 0usize..8 {
        for u in 0usize..8 {
          let cucv = if u == 0 && v == 0 {
            1.0_f64 / 2.0_f64.sqrt()
          }else{
            1.0_f64
          };
          dctcoef.put_pixel(
            blockx * 8 + u as u32, blocky * 8 + v as u32, 
            image::Luma([(0.25_f64 * cucv * innerprod(&subimage, &costmpl[v][u])) as u8])
            );
        }
      }
    }
  }

  println!("{:?}", dctcoef.save("dctcoef.png"));
}
