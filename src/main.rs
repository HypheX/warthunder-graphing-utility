

use best_fit::fit;
use plotters::{prelude::*, style::full_palette::GREEN_700};


mod best_fit;
mod stuff;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cutoff = 0.015;

    let root = BitMapBackend::new("output/sep_per_alt_2.png", (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("La-7", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(45)
        .build_cartesian_2d(0.0..650.0,  -5.0..30.0)?;

    chart
        .configure_mesh()
        .x_desc("IAS, km/h")
        .y_desc("Specific Excess Power, m/s")
        .draw()?;


    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (1000.0, 0.0)], 
        &full_palette::GREY_700
    ))?;


    let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
    let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
    chart   
        .draw_series(   //200
            data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 100.0) | (alt > 300.0) {return None}
                Some(Circle::new((ias, sep), 2, 
                ShapeStyle {
                    color: RED.mix(0.3),
                    filled: true,
                    stroke_width: 1,
                }))
            })
        )?
        .label("200m")
        .legend(|(x, y)| Circle::new((x+10, y), 2, 
        ShapeStyle {
            color: RED.mix(0.8),
            filled: true,
            stroke_width: 1,
        }));


    let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
    let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
    chart  
        .draw_series(LineSeries::new(
            fit(data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 100.0) | (alt > 300.0) {return None}
                Some((ias, sep))
            }).collect(), 650.0, cutoff),
            RED,
        ))?;



    let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
    let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
    chart   
        .draw_series(   //2000
            data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 1900.0) | (alt > 2100.0) {return None}
                Some(Circle::new((ias, sep), 2, 
                ShapeStyle {
                    color: GREEN_700.mix(0.3),
                    filled: true,
                    stroke_width: 1,
                }))
            })
        )?
        .label("2000m")
        .legend(|(x, y)| Circle::new((x+10, y), 2, 
        ShapeStyle {
            color: GREEN_700.mix(0.8),
            filled: true,
            stroke_width: 1,
        }));


    let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
    let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
    chart  
        .draw_series(LineSeries::new(
            fit(data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 1900.0) | (alt > 2100.0) {return None}
                Some((ias, sep))
            }).collect(), 650.0, cutoff),
            GREEN_700,
        ))?;


    let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
    let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
    chart   
        .draw_series(   //6000
            data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 5900.0) | (alt > 6100.0) {return None}
                Some(Circle::new((ias, sep), 2, 
                ShapeStyle {
                    color: BLUE.mix(0.3),
                    filled: true,
                    stroke_width: 1,
                }))
            })
        )?
        .label("6000m")
        .legend(|(x, y)| Circle::new((x+10, y), 2, 
        ShapeStyle {
            color: BLUE.mix(0.8),
            filled: true,
            stroke_width: 1,
        }));


    let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
    let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
    chart  
        .draw_series(LineSeries::new(
            fit(data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 5900.0) | (alt > 6100.0) {return None}
                Some((ias, sep))
            }).collect(), 650.0, cutoff),
            BLUE,
        ))?;


    /*
    
     */

    /* 
    let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
    let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
    chart   //200m
        .draw_series(
            data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 100.0) | (alt > 300.0) {return None}
                Some(Circle::new((ias, sep), 2, 
                ShapeStyle {
                    color: RED.mix(0.5),
                    filled: true,
                    stroke_width: 1,
                }))
            })
        )?
        .label("200m")
        .legend(|(x, y)| Circle::new((x+5, y), 2, 
        ShapeStyle {
            color: RED.mix(0.5),
            filled: true,
            stroke_width: 1,
        }));

    
        let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
        let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
        chart   //2000m
        .draw_series(
            data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 1900.0) | (alt > 2100.0) {return None}
                Some(Circle::new((ias, sep), 2, 
                ShapeStyle {
                    color: full_palette::GREEN_700.mix(0.5),
                    filled: true,
                    stroke_width: 1,
                }))
            })
        )?
        .label("2000m")
        .legend(|(x, y)| Circle::new((x+5, y), 2, 
        ShapeStyle {
            color: full_palette::GREEN_700.mix(0.5),
            filled: true,
            stroke_width: 1,
        }));


        let mut reader = csv::Reader::from_path("input/sep_per_alt.csv")?;
        let data_iter: csv::DeserializeRecordsIter<'_, std::fs::File, (f64, f64, f64, f64, f64)> = reader.deserialize();
        chart   //6000m
        .draw_series(
            data_iter.filter_map(|point| {
                let point = point.unwrap();
                let (_time, ias, sep, thr, alt) = point;
                if (thr < 110.0) | (alt < 5900.0) | (alt > 6100.0) {return None}
                Some(Circle::new((ias, sep), 2, 
                ShapeStyle {
                    color: BLUE.mix(0.5),
                    filled: true,
                    stroke_width: 1,
                }))
            })
        )?
        .label("6000m")
        .legend(|(x, y)| Circle::new((x+5, y), 2, 
        ShapeStyle {
            color: BLUE.mix(0.5),
            filled: true,
            stroke_width: 1,
        }));
    */


    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    


    root.present()?;

    Ok(())
}


