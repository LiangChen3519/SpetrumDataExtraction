var roi = 
    /* color: #d63000 */
    /* displayProperties: [
      {
        "type": "rectangle"
      }
    ] */
ee.Geometry.Polygon(
        [[[29.549129639679126, 63.23267098489592],
          [29.549129639679126, 62.99793541172316],
          [30.197322999054126, 62.99793541172316],
          [30.197322999054126, 63.23267098489592]]], null, false),
landsat8 = ee.ImageCollection("LANDSAT/LC08/C02/T1_TOA");
var img_best = landsat8
                .filterBounds(roi)
                .filterDate('2020-01-01','2025-05-31');
                //.sort('CLOUD_COVER')
                //.first();
//print(img_best);

var visPara = {bands:['B4','B3','B2'],max:0.3};
Map.addLayer(img_best,visPara,'best one');

// ndvi = (b5-b4)/(b5+b4) = (NIR -RED)/(NIR + RED)
//var ndvi = img_best.normalizedDifference(['B5','B4']).rename('NDVI');
//var ndvi_roi = ndvi.clip(roi);
//var visPara1 = {min:-1,max:1,palette:['blue','white','green']};
//Map.addLayer(ndvi_roi,visPara1,'NDVI');

// time series of the NDVI

var addNdvi = function(img){
  var ndvi = img.normalizedDifference(['B5','B4']).rename('NDVI').clip(roi);
  return img.addBands(ndvi);
}

var imgs_ndvi = img_best.map(addNdvi);
//print(imgs_ndvi);

// plot ndvi in last 5 years
var ndviChart = ui.Chart.image.series(
  {
    imageCollection:imgs_ndvi.select('NDVI'),
    region:roi,
    reducer:ee.Reducer.mean(),
    scale:30,
  }
  );
print(ndviChart);