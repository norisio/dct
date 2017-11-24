extern crate image;

use image::{GenericImage, Pixel, Luma};
use std::f64::consts;


fn innerprod<I, J>(subimage: &I, costmpl: &J) -> f64 
  where I: GenericImage<Pixel = Luma<u8>>,
        J: GenericImage<Pixel = Luma<f64>>
  {
  let mut result = 0.0_f64;
  for y in 0..8{
    for x in 0..8 {
      result += 
        subimage.get_pixel(x, y).to_luma().data[0] as f64 *
        costmpl.get_pixel(x, y).to_luma().data[0];
    }
  }
  result
}

fn main() {
  let mut img = image::open("map_gray_crop.png").unwrap().to_luma();
  let (dimx, dimy) = img.dimensions();
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
       * To dump dct base vectors
       *
      let tr = image::ImageBuffer::from_fn(8, 8, |x, y| {
        image::Luma([(costmpl_u[u].get_pixel(x, y).to_luma().data[0] * 128_f64 + 127_f64) as u8])
      });
      tr.save(format!("costmpl{}{}.png", u, v));
      */
    }
    costmpl.push(costmpl_u);
  }

  let costmpl = costmpl;
  let mut dctcoef :image::ImageBuffer<Luma<f64>, _> = image::ImageBuffer::new(dimx, dimy);

  for blocky in 0..repy {
    for blockx in 0..repx{
      let subimage = img.sub_image(blockx*8, blocky*8, 8, 8);
      for v in 0usize..8 {
        for u in 0usize..8 {
          let cucv = if u == 0usize && v == 0usize {
            1.0_f64 / 2.0_f64.sqrt()
          }else{
            1.0_f64
          };
          dctcoef.put_pixel(
            blockx * 8 + u as u32, blocky * 8 + v as u32, 
            image::Luma([(0.25_f64 * cucv * innerprod(&subimage, &costmpl[v][u])) as f64])
            );
        }
      }
    }
  }

  // println!("save dctcoef.png : {:?}", dctcoef.save("dctcoef.png"));

  let mut sums = [[0.0f64; 8]; 8];
  for y in 0..dimy {
    for x in 0..dimx{
      let v = (y % 8) as usize;
      let u = (x % 8) as usize;
      sums[v][u] += dctcoef.get_pixel(x, y).data[0] as f64;
    }
  }
  let sums = sums;
  println!("sums:");
  for v in 0usize..8 {
    for u in 0usize..8 {
      print!("{:.2}", sums[v][u]);
      if u != 7 {
        print!(", ");
      }
    }
    println!("");
  }
  let mut means = [[0.0f64; 8]; 8];
  let denominator = (repx * repy) as f64;
  for v in 0usize..8 {
    for u in 0usize..8 {
      means[v][u] = sums[v][u] as f64 / denominator;
    }
  }
  let means = means;
  println!("means:");
  for v in 0usize..8 {
    for u in 0usize..8 {
      print!("{:.2}", means[v][u]);
      if u != 7 {
        print!(", ");
      }
    }
    println!("");
  }

  let mut sqerrors = [[0.0f64; 8]; 8];
  for y in 0..dimy {
    for x in 0..dimx{
      let v = (y % 8) as usize;
      let u = (x % 8) as usize;
      let coef = dctcoef.get_pixel(x, y).data[0] as f64;
      let mean = means[v][u];

      sqerrors[v][u] += (coef - mean).powf(2.0f64);
      if v ==0 && u == 0{
        println!("({} - {})^2 = {}",  coef, mean, (coef-mean).powf(2.0f64));
      }
    }
  }
  let sqerrors = sqerrors;

  println!("sqerrors:");
  for v in 0usize..8 {
    for u in 0usize..8 {
      print!("{:.2}", sqerrors[v][u]);
      if u != 7 {
        print!(", ");
      }
    }
    println!("");
  }


  let mut variances = [[0.0f64; 8]; 8];
  for v in 0usize..8 {
    for u in 0usize..8 {
      variances[v][u] = sqerrors[v][u] / denominator;
    }
  }
  let variances = variances;


  println!("variances:");
  for v in 0usize..8 {
    for u in 0usize..8 {
      print!("{:.2}", variances[v][u]);
      if u != 7 {
        print!(", ");
      }
    }
    println!("");
  }

  let mut stderrs = [[0.0f64; 8]; 8];
  for v in 0usize..8 {
    for u in 0usize..8 {
      stderrs[v][u] = variances[v][u].sqrt();
    }
  }

  println!("stderrs:");
  for v in 0usize..8 {
    for u in 0usize..8 {
      print!("{:.2}", stderrs[v][u]);
      if u != 7 {
        print!(", ");
      }
    }
    println!("");
  }
}

