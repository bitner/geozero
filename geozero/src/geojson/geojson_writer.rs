use crate::error::Result;
use crate::{ColumnValue, CoordDimensions, FeatureProcessor, GeomProcessor, PropertyProcessor};
use std::fmt::Display;
use std::io::Write;

/// GeoJSON writer.
pub struct GeoJsonWriter<W: Write> {
    dims: CoordDimensions,
    out: W,
}

impl<W: Write> GeoJsonWriter<W> {
    pub fn new(out: W) -> Self {
        GeoJsonWriter {
            dims: CoordDimensions::default(),
            out,
        }
    }
    pub fn with_dims(out: W, dims: CoordDimensions) -> Self {
        GeoJsonWriter { dims, out }
    }
    fn comma(&mut self, idx: usize) -> Result<()> {
        if idx > 0 {
            self.out.write_all(b",")?;
        }
        Ok(())
    }
}

impl<W: Write> FeatureProcessor for GeoJsonWriter<W> {
    fn dataset_begin(&mut self, name: Option<&str>) -> Result<()> {
        self.out.write_all(
            br#"{
"type": "FeatureCollection""#,
        )?;
        if let Some(name) = name {
            write!(self.out, ",\n\"name\": \"{name}\"")?;
        }
        self.out.write_all(
            br#",
"features": ["#,
        )?;
        Ok(())
    }
    fn dataset_end(&mut self) -> Result<()> {
        self.out.write_all(b"]}")?;
        Ok(())
    }
    fn feature_begin(&mut self, idx: u64) -> Result<()> {
        if idx > 0 {
            self.out.write_all(b",\n")?;
        }
        self.out.write_all(br#"{"type": "Feature""#)?;
        Ok(())
    }
    fn feature_end(&mut self, _idx: u64) -> Result<()> {
        self.out.write_all(b"}")?;
        Ok(())
    }
    fn properties_begin(&mut self) -> Result<()> {
        self.out.write_all(br#", "properties": {"#)?;
        Ok(())
    }
    fn properties_end(&mut self) -> Result<()> {
        self.out.write_all(b"}")?;
        Ok(())
    }
    fn geometry_begin(&mut self) -> Result<()> {
        self.out.write_all(br#", "geometry": "#)?;
        Ok(())
    }
    fn geometry_end(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<W: Write> GeomProcessor for GeoJsonWriter<W> {
    fn dimensions(&self) -> CoordDimensions {
        self.dims
    }
    fn xy(&mut self, x: f64, y: f64, idx: usize) -> Result<()> {
        self.comma(idx)?;
        self.out.write_all(format!("[{x},{y}]").as_bytes())?;
        Ok(())
    }
    fn coordinate(
        &mut self,
        x: f64,
        y: f64,
        z: Option<f64>,
        _m: Option<f64>,
        _t: Option<f64>,
        _tm: Option<u64>,
        idx: usize,
    ) -> Result<()> {
        self.comma(idx)?;
        self.out.write_all(format!("[{x},{y}").as_bytes())?;
        if let Some(z) = z {
            self.out.write_all(format!(",{z}").as_bytes())?;
        }
        self.out.write_all(b"]")?;
        Ok(())
    }
    fn point_begin(&mut self, idx: usize) -> Result<()> {
        self.comma(idx)?;
        self.out
            .write_all(br#"{"type": "Point", "coordinates": "#)?;
        Ok(())
    }
    fn point_end(&mut self, _idx: usize) -> Result<()> {
        self.out.write_all(b"}")?;
        Ok(())
    }
    fn multipoint_begin(&mut self, _size: usize, idx: usize) -> Result<()> {
        self.comma(idx)?;
        self.out
            .write_all(br#"{"type": "MultiPoint", "coordinates": ["#)?;
        Ok(())
    }
    fn multipoint_end(&mut self, _idx: usize) -> Result<()> {
        self.out.write_all(b"]}")?;
        Ok(())
    }
    fn linestring_begin(&mut self, tagged: bool, _size: usize, idx: usize) -> Result<()> {
        self.comma(idx)?;
        if tagged {
            self.out
                .write_all(br#"{"type": "LineString", "coordinates": ["#)?;
        } else {
            self.out.write_all(b"[")?;
        }
        Ok(())
    }
    fn linestring_end(&mut self, tagged: bool, _idx: usize) -> Result<()> {
        if tagged {
            self.out.write_all(b"]}")?;
        } else {
            self.out.write_all(b"]")?;
        }
        Ok(())
    }
    fn multilinestring_begin(&mut self, _size: usize, idx: usize) -> Result<()> {
        self.comma(idx)?;
        self.out
            .write_all(br#"{"type": "MultiLineString", "coordinates": ["#)?;
        Ok(())
    }
    fn multilinestring_end(&mut self, _idx: usize) -> Result<()> {
        self.out.write_all(b"]}")?;
        Ok(())
    }
    fn polygon_begin(&mut self, tagged: bool, _size: usize, idx: usize) -> Result<()> {
        self.comma(idx)?;
        if tagged {
            self.out
                .write_all(br#"{"type": "Polygon", "coordinates": ["#)?;
        } else {
            self.out.write_all(b"[")?;
        }
        Ok(())
    }
    fn polygon_end(&mut self, tagged: bool, _idx: usize) -> Result<()> {
        if tagged {
            self.out.write_all(b"]}")?;
        } else {
            self.out.write_all(b"]")?;
        }
        Ok(())
    }
    fn multipolygon_begin(&mut self, _size: usize, idx: usize) -> Result<()> {
        self.comma(idx)?;
        self.out
            .write_all(br#"{"type": "MultiPolygon", "coordinates": ["#)?;
        Ok(())
    }
    fn multipolygon_end(&mut self, _idx: usize) -> Result<()> {
        self.out.write_all(b"]}")?;
        Ok(())
    }
    fn geometrycollection_begin(&mut self, _size: usize, idx: usize) -> Result<()> {
        self.comma(idx)?;
        self.out
            .write_all(br#"{"type": "GeometryCollection", "geometries": ["#)?;
        Ok(())
    }
    fn geometrycollection_end(&mut self, _idx: usize) -> Result<()> {
        self.out.write_all(b"]}")?;
        Ok(())
    }
}

fn write_num_prop<W: Write>(mut out: W, colname: &str, v: &dyn Display) -> Result<()> {
    let colname = colname.replace('\"', "\\\"");
    out.write_all(format!(r#""{colname}": {v}"#).as_bytes())?;
    Ok(())
}

fn write_str_prop<W: Write>(mut out: W, colname: &str, v: &str) -> Result<()> {
    let colname = colname.replace('\"', "\\\"");
    let value = v.replace('\"', "\\\"");
    out.write_all(format!(r#""{colname}": "{value}""#).as_bytes())?;
    Ok(())
}

impl<W: Write> PropertyProcessor for GeoJsonWriter<W> {
    fn property(&mut self, i: usize, colname: &str, colval: &ColumnValue) -> Result<bool> {
        if i > 0 {
            self.out.write_all(b", ")?;
        }
        match colval {
            ColumnValue::Byte(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::UByte(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::Bool(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::Short(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::UShort(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::Int(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::UInt(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::Long(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::ULong(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::Float(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::Double(v) => write_num_prop(&mut self.out, colname, &v)?,
            ColumnValue::String(v) | ColumnValue::DateTime(v) => {
                write_str_prop(&mut self.out, colname, v)?;
            }
            ColumnValue::Json(_v) => (),
            ColumnValue::Binary(_v) => (),
        };
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geojson::read_geojson;
    use crate::ToJson;

    #[test]
    fn geometries() -> Result<()> {
        // countries.fgb, id = ZAF
        let geojson = r#"{
            "type": "MultiPolygon",
            "coordinates": [[[
                [31.521001,-29.257387],[31.325561,-29.401978],[30.901763,-29.909957],[30.622813,-30.423776],[30.055716,-31.140269],[28.925553,-32.172041],[28.219756,-32.771953],[27.464608,-33.226964],[26.419452,-33.61495],[25.909664,-33.66704],[25.780628,-33.944646],[25.172862,-33.796851],[24.677853,-33.987176],[23.594043,-33.794474],[22.988189,-33.916431],[22.574157,-33.864083],[21.542799,-34.258839],[20.689053,-34.417175],[20.071261,-34.795137],[19.616405,-34.819166],[19.193278,-34.462599],[18.855315,-34.444306],[18.424643,-33.997873],[18.377411,-34.136521],[18.244499,-33.867752],[18.25008,-33.281431],[17.92519,-32.611291],[18.24791,-32.429131],[18.221762,-31.661633],[17.566918,-30.725721],[17.064416,-29.878641],[17.062918,-29.875954],[16.344977,-28.576705],[16.824017,-28.082162],[17.218929,-28.355943],[17.387497,-28.783514],[17.836152,-28.856378],[18.464899,-29.045462],[19.002127,-28.972443],[19.894734,-28.461105],[19.895768,-24.76779],[20.165726,-24.917962],[20.758609,-25.868136],[20.66647,-26.477453],[20.889609,-26.828543],[21.605896,-26.726534],[22.105969,-26.280256],[22.579532,-25.979448],[22.824271,-25.500459],[23.312097,-25.26869],[23.73357,-25.390129],[24.211267,-25.670216],[25.025171,-25.71967],[25.664666,-25.486816],[25.765849,-25.174845],[25.941652,-24.696373],[26.485753,-24.616327],[26.786407,-24.240691],[27.11941,-23.574323],[28.017236,-22.827754],[29.432188,-22.091313],[29.839037,-22.102216],[30.322883,-22.271612],[30.659865,-22.151567],[31.191409,-22.25151],[31.670398,-23.658969],[31.930589,-24.369417],[31.752408,-25.484284],[31.837778,-25.843332],[31.333158,-25.660191],[31.04408,-25.731452],[30.949667,-26.022649],[30.676609,-26.398078],[30.685962,-26.743845],[31.282773,-27.285879],[31.86806,-27.177927],[32.071665,-26.73382],[32.83012,-26.742192],[32.580265,-27.470158],[32.462133,-28.301011],[32.203389,-28.752405],[31.521001,-29.257387]
            ],[
                [28.978263,-28.955597],[28.5417,-28.647502],[28.074338,-28.851469],[27.532511,-29.242711],[26.999262,-29.875954],[27.749397,-30.645106],[28.107205,-30.545732],[28.291069,-30.226217],[28.8484,-30.070051],[29.018415,-29.743766],[29.325166,-29.257387],[28.978263,-28.955597]
            ]]]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        // Has Multi-Ring Polygon
        assert_json_eq(&out, geojson);

        // countries.fgb, id = NZL
        let geojson = r#"{
            "type": "MultiPolygon",
            "coordinates": [[[
                [173.020375,-40.919052],[173.247234,-41.331999],[173.958405,-40.926701],[174.247587,-41.349155],[174.248517,-41.770008],[173.876447,-42.233184],[173.22274,-42.970038],[172.711246,-43.372288],[173.080113,-43.853344],[172.308584,-43.865694],[171.452925,-44.242519],[171.185138,-44.897104],[170.616697,-45.908929],[169.831422,-46.355775],[169.332331,-46.641235],[168.411354,-46.619945],[167.763745,-46.290197],[166.676886,-46.219917],[166.509144,-45.852705],[167.046424,-45.110941],[168.303763,-44.123973],[168.949409,-43.935819],[169.667815,-43.555326],[170.52492,-43.031688],[171.12509,-42.512754],[171.569714,-41.767424],[171.948709,-41.514417],[172.097227,-40.956104],[172.79858,-40.493962],[173.020375,-40.919052]
            ]],[[
                [174.612009,-36.156397],[175.336616,-37.209098],[175.357596,-36.526194],[175.808887,-36.798942],[175.95849,-37.555382],[176.763195,-37.881253],[177.438813,-37.961248],[178.010354,-37.579825],[178.517094,-37.695373],[178.274731,-38.582813],[177.97046,-39.166343],[177.206993,-39.145776],[176.939981,-39.449736],[177.032946,-39.879943],[176.885824,-40.065978],[176.508017,-40.604808],[176.01244,-41.289624],[175.239567,-41.688308],[175.067898,-41.425895],[174.650973,-41.281821],[175.22763,-40.459236],[174.900157,-39.908933],[173.824047,-39.508854],[173.852262,-39.146602],[174.574802,-38.797683],[174.743474,-38.027808],[174.697017,-37.381129],[174.292028,-36.711092],[174.319004,-36.534824],[173.840997,-36.121981],[173.054171,-35.237125],[172.636005,-34.529107],[173.007042,-34.450662],[173.551298,-35.006183],[174.32939,-35.265496],[174.612009,-36.156397]
            ]]]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        // Has multiple Polygons
        assert_json_eq(&out, geojson);

        // lines.fgb, first feature
        let geojson = r#"{
            "type": "LineString",
            "coordinates": [
                [1875038.4476102313,-3269648.6879248763],[1874359.6415041967,-3270196.8129848638],[1874141.0428635243,-3270953.7840121365],[1874440.1778162003,-3271619.4315206874],[1876396.0598222911,-3274138.747656357],[1876442.0805243007,-3275052.60551469],[1874739.312657555,-3275457.333765534]
            ]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        assert_json_eq(&out, geojson);

        // ne_10m_geographic_lines.fgb, first feature
        let geojson = r#"{
            "type": "MultiLineString",
            "coordinates": [[
                [-20037505.025679983,2692596.21474788],[-19924286.672913034,2692596.21474788],[-19812966.14702537,2692596.21474788],[-19701645.62113772,2692596.21474788],[-19590325.09525006,2692596.21474788],[-19479004.56936241,2692596.21474788],[-19367684.04347475,2692596.21474788],[-19256372.576874677,2692596.21474788],[-19145042.991699435,2692596.21474788],[-19033736.054743163,2692596.21474788],[-18922401.939924125,2692596.21474788],[-18811090.473324053,2692596.21474788],[-18699774.4770802,2692596.21474788],[-18588449.421548743,2692596.21474788],[-18477128.895661093,2692596.21474788],[-18365812.89941723,2692596.21474788],[-18254492.373529565,2692596.21474788],[-18143167.31799812,2692596.21474788],[-18031860.381041847,2692596.21474788],[-17920530.795866605,2692596.21474788],[-17809219.329266533,2692596.21474788],[-17697898.803378873,2692596.21474788],[-17586578.277491223,2692596.21474788],[-17475257.751603562,2692596.21474788],[-17363937.225715913,2692596.21474788],[-17252616.69982825,2692596.21474788],[-17141296.1739406,2692596.21474788],[-17029984.707340535,2692596.21474788],[-16918659.651809078,2692596.21474788],[-16807343.65556522,2692596.21474788],[-16696014.07038997,2692596.21474788],[-16584707.133433694,2692596.21474788],[-16473382.077902246,2692596.21474788],[-16362061.552014597,2692596.21474788],[-16250745.55577073,2692596.21474788],[-16139425.02988307,2692596.21474788],[-16028104.503995419,2692596.21474788],[-15916783.978107756,2692596.21474788],[-15805472.511507692,2692596.21474788],[-15694142.926332444,2692596.21474788],[-15582831.459732382,2692596.21474788],[-15471510.933844728,2692596.21474788],[-15360190.407957068,2692596.21474788],[-15248869.882069414,2692596.21474788],[-15137549.356181756,2692596.21474788],[-15026228.830294106,2692596.21474788],[-14914908.304406442,2692596.21474788],[-14803587.778518781,2692596.21474788],[-14692267.252631132,2692596.21474788],[-14580955.78603106,2692596.21474788],[-14469630.730499614,2692596.21474788],[-14358314.734255752,2692596.21474788],[-14246994.20836809,2692596.21474788],[-14135678.212124234,2692596.21474788],[-14024353.156592779,2692596.21474788],[-13913032.630705126,2692596.21474788],[-13801716.634461263,2692596.21474788],[-13690391.578929815,2692596.21474788],[-13579080.112329746,2692596.21474788],[-13467755.05679829,2692596.21474788],[-13356439.060554435,2692596.21474788],[-13245114.005022977,2692596.21474788],[-13133802.538422907,2692596.21474788],[-13022482.012535257,2692596.21474788],[-12911161.4866476,2692596.21474788],[-12799840.960759947,2692596.21474788],[-12688520.434872286,2692596.21474788],[-12577199.908984635,2692596.21474788],[-12465883.912740769,2692596.21474788],[-12354567.916496906,2692596.21474788],[-12243238.33132166,2692596.21474788],[-12131926.864721594,2692596.21474788],[-12020601.809190147,2692596.21474788],[-11909285.812946282,2692596.21474788],[-11797969.816702416,2692596.21474788],[-11686644.76117097,2692596.21474788],[-11575328.764927106,2692596.21474788],[-11464008.239039455,2692596.21474788],[-11352687.713151794,2692596.21474788],[-11241367.187264143,2692596.21474788],[-11130051.191020276,2692596.21474788],[-11018726.13548883,2692596.21474788],[-10907414.668888763,2692596.21474788],[-10796094.1430011,2692596.21474788],[-10684773.61711345,2692596.21474788],[-10573453.091225792,2692596.21474788],[-10462132.56533813,2692596.21474788],[-10350812.03945048,2692596.21474788],[-10239491.513562817,2692596.21474788],[-10128170.987675166,2692596.21474788],[-10016854.9914313,2692596.21474788],[-9905538.995187437,2692596.21474788],[-9794209.410012191,2692596.21474788],[-9682902.47305592,2692596.21474788],[-9571577.417524474,2692596.21474788],[-9460261.421280608,2692596.21474788],[-9348940.895392958,2692596.21474788],[-9237615.8398615,2692596.21474788],[-9126299.843617637,2692596.21474788],[-9014979.317729987,2692596.21474788],[-8903663.321486121,2692596.21474788],[-8792338.265954675,2692596.21474788],[-8681022.269710807,2692596.21474788],[-8569692.684535567,2692596.21474788],[-8458385.747579295,2692596.21474788],[-8347069.751335428,2692596.21474788],[-8235749.225447779,2692596.21474788],[-8124424.16991632,2692596.21474788],[-8013099.114384874,2692596.21474788],[-7901787.647784806,2692596.21474788],[-7790467.121897143,2692596.21474788],[-7679155.655297086,2692596.21474788],[-7567826.070121832,2692596.21474788],[-7456510.073877977,2692596.21474788],[-7345185.01834652,2692596.21474788],[-7233873.551746452,2692596.21474788],[-7122553.025858803,2692596.21474788],[-7011232.49997114,2692596.21474788],[-6899911.97408349,2692596.21474788],[-6788586.918552041,2692596.21474788],[-6677270.922308178,2692596.21474788],[-6565950.396420515,2692596.21474788],[-6454638.929820447,2692596.21474788],[-6343309.344645206,2692596.21474788],[-6231993.34840134,2692596.21474788],[-6120677.352157486,2692596.21474788],[-6009356.826269826,2692596.21474788],[-5898040.83002596,2692596.21474788],[-5786715.774494514,2692596.21474788],[-5675395.248606861,2692596.21474788],[-5564074.722719202,2692596.21474788],[-5452754.196831549,2692596.21474788],[-5341433.67094389,2692596.21474788],[-5230126.733987618,2692596.21474788],[-5118797.148812373,2692596.21474788],[-5007481.15256851,2692596.21474788],[-4896165.156324643,2692596.21474788],[-4784844.630436993,2692596.21474788],[-4673524.104549334,2692596.21474788],[-4562203.578661681,2692596.21474788],[-4450883.052774021,2692596.21474788],[-4339562.526886369,2692596.21474788],[-4228242.000998709,2692596.21474788],[-4116935.0640424383,2692596.21474788],[-4005600.9492233973,2692596.21474788],[-3894289.48262333,2692596.21474788],[-3782964.427091881,2692596.21474788],[-3671648.4308480173,2692596.21474788],[-3560327.904960355,2692596.21474788],[-3449011.908716501,2692596.21474788],[-3337691.382828842,2692596.21474788],[-3226366.327297393,2692596.21474788],[-3115050.3310535294,2692596.21474788],[-3003729.8051658766,2692596.21474788],[-2892418.338565809,2692596.21474788],[-2781088.7533905646,2692596.21474788],[-2669777.2867904967,2692596.21474788],[-2558456.7609028374,2692596.21474788],[-2447136.2350151846,2692596.21474788],[-2335815.7091275253,2692596.21474788],[-2224495.183239872,2692596.21474788],[-2113174.657352213,2692596.21474788],[-2001854.1314645505,2692596.21474788],[-1890533.6055769008,2692596.21474788],[-1779213.0796892412,2692596.21474788],[-1667901.6130891705,2692596.21474788],[-1556581.087201521,2692596.21474788],[-1445260.5613138585,2692596.21474788],[-1333940.0354262087,2692596.21474788],[-1222619.5095385492,2692596.21474788],[-1111298.9836508965,2692596.21474788],[-999978.457763237,2692596.21474788],[-888662.4615193801,2692596.21474788],[-777341.9356317207,2692596.21474788],[-666025.9393878573,2692596.21474788],[-554696.3542126124,2692596.21474788],[-443393.94690013694,2692596.21474788],[-332073.4210124745,2692596.21474788],[-220743.83583723273,2692596.21474788],[-109432.36923716537,2692596.21474788],[1897.2159380795622,2692596.21474788],[113226.80111332452,2692596.21474788],[224538.26771339186,2692596.21474788],[335876.91217622877,2692596.21474788],[447188.37877629616,2692596.21474788],[558499.8453763635,2692596.21474788],[669820.3712640165,2692596.21474788],[781140.8971516758,2692596.21474788],[892461.4230393288,2692596.21474788],[1003772.8896393961,2692596.21474788],[1115111.534102233,2692596.21474788],[1226404.882127126,2692596.21474788],[1337743.52658995,2692596.21474788],[1449064.0524776129,2692596.21474788],[1560384.5783652721,2692596.21474788],[1671696.04496533,2692596.21474788],[1783007.5115654003,2692596.21474788],[1894328.0374530598,2692596.21474788],[2005648.5633407128,2692596.21474788],[2116978.148515964,2692596.21474788],[2228289.6151160216,2692596.21474788],[2339619.2002912764,2692596.21474788],[2450930.666891334,2692596.21474788],[2562251.1927789967,2692596.21474788],[2673571.718666656,2692596.21474788],[2784901.303841901,2692596.21474788],[2896212.7704419685,2692596.21474788],[3007515.1777544436,2692596.21474788],[3118853.822217281,2692596.21474788],[3230165.2888173484,2692596.21474788],[3341494.8739925832,2692596.21474788],[3452797.2813050686,2692596.21474788],[3564135.9257678958,2692596.21474788],[3675438.333080381,2692596.21474788],[3786776.9775432083,2692596.21474788],[3898088.444143285,2692596.21474788],[4009399.9107433553,2692596.21474788],[4120720.4366310053,2692596.21474788],[4232040.962518668,2692596.21474788],[4343361.488406317,2692596.21474788],[4454672.9550063815,2692596.21474788],[4566011.599469221,2692596.21474788],[4677323.066069286,2692596.21474788],[4788643.591956948,2692596.21474788],[4899964.117844598,2692596.21474788],[5011284.643732261,2692596.21474788],[5122596.110332319,2692596.21474788],[5233907.576932389,2692596.21474788],[5345246.221395223,2692596.21474788],[5456548.628707701,2692596.21474788],[5567887.273170535,2692596.21474788],[5679189.680483013,2692596.21474788],[5790519.265658256,2692596.21474788],[5901830.732258332,2692596.21474788],[6013151.258145982,2692596.21474788],[6124471.784033645,2692596.21474788],[6235801.36920888,2692596.21474788],[6347112.835808957,2692596.21474788],[6458433.3616966065,2692596.21474788],[6569753.887584269,2692596.21474788],[6681074.413471919,2692596.21474788],[6792394.939359581,2692596.21474788],[6903706.405959652,2692596.21474788],[7015035.991134894,2692596.21474788],[7126356.517022544,2692596.21474788],[7237677.042910206,2692596.21474788],[7348988.509510271,2692596.21474788],[7460299.976110341,2692596.21474788],[7571620.501997991,2692596.21474788],[7682941.027885654,2692596.21474788],[7794261.5537733035,2692596.21474788],[7905573.020373374,2692596.21474788],[8016911.664836207,2692596.21474788],[8128223.131436277,2692596.21474788],[8239543.657323928,2692596.21474788],[8350864.18321159,2692596.21474788],[8462184.70909924,2692596.21474788],[8573496.175699318,2692596.21474788],[8684807.642299388,2692596.21474788],[8796146.286762215,2692596.21474788],[8907457.753362292,2692596.21474788],[9018787.338537533,2692596.21474788],[9130098.805137604,2692596.21474788],[9241419.331025254,2692596.21474788],[9352730.797625326,2692596.21474788],[9464069.442088157,2692596.21474788],[9575371.849400638,2692596.21474788],[9686701.434575878,2692596.21474788],[9798012.901175942,2692596.21474788],[9909333.427063597,2692596.21474788],[10020653.952951254,2692596.21474788],[10131965.419551326,2692596.21474788],[10243295.004726568,2692596.21474788],[10354606.471326638,2692596.21474788],[10465936.05650188,2692596.21474788],[10577256.58238953,2692596.21474788],[10688577.108277192,2692596.21474788],[10799888.574877262,2692596.21474788],[10911218.160052504,2692596.21474788],[11022520.567364983,2692596.21474788],[11133841.093252633,2692596.21474788],[11245179.737715466,2692596.21474788],[11356482.145027963,2692596.21474788],[11467811.730203198,2692596.21474788],[11579123.196803275,2692596.21474788],[11690443.722690927,2692596.21474788],[11801764.24857859,2692596.21474788],[11913093.833753832,2692596.21474788],[12024405.300353901,2692596.21474788],[12135707.707666373,2692596.21474788],[12247046.352129214,2692596.21474788],[12358357.818729272,2692596.21474788],[12469687.403904526,2692596.21474788],[12580998.870504584,2692596.21474788],[12692319.39639224,2692596.21474788],[12803630.862992309,2692596.21474788],[12914969.507455144,2692596.21474788],[13026280.9740552,2692596.21474788],[13137610.559230454,2692596.21474788],[13248912.966542935,2692596.21474788],[13360233.492430585,2692596.21474788],[13471554.018318245,2692596.21474788],[13582865.484918306,2692596.21474788],[13694204.129381137,2692596.21474788],[13805515.595981209,2692596.21474788],[13916836.121868871,2692596.21474788],[14028156.64775652,2692596.21474788],[14139477.173644185,2692596.21474788],[14250788.640244242,2692596.21474788],[14362127.284707077,2692596.21474788],[14473429.692019572,2692596.21474788],[14584741.158619631,2692596.21474788],[14696079.803082459,2692596.21474788],[14807382.210394945,2692596.21474788],[14918711.795570198,2692596.21474788],[15030023.262170255,2692596.21474788],[15141343.78805791,2692596.21474788],[15252664.313945567,2692596.21474788],[15364002.9584084,2692596.21474788],[15475305.365720881,2692596.21474788],[15586634.950896129,2692596.21474788],[15697946.417496186,2692596.21474788],[15809257.884096257,2692596.21474788],[15920587.4692715,2692596.21474788],[16031898.93587157,2692596.21474788],[16143228.52104681,2692596.21474788],[16254549.046934472,2692596.21474788],[16365869.572822122,2692596.21474788],[16477181.039422194,2692596.21474788],[16588510.624597436,2692596.21474788],[16699822.091197504,2692596.21474788],[16811151.676372748,2692596.21474788],[16922454.083685227,2692596.21474788],[17033765.550285302,2692596.21474788],[17145104.19474813,2692596.21474788],[17256415.661348205,2692596.21474788],[17367736.18723587,2692596.21474788],[17479056.71312352,2692596.21474788],[17590377.239011183,2692596.21474788],[17701688.70561124,2692596.21474788],[17813027.350074075,2692596.21474788],[17924338.816674147,2692596.21474788],[18035659.3425618,2692596.21474788],[18146979.868449457,2692596.21474788],[18258282.27576193,2692596.21474788],[18369611.86093717,2692596.21474788],[18480923.327537242,2692596.21474788],[18592261.97200008,2692596.21474788],[18703564.379312553,2692596.21474788],[18814903.023775388,2692596.21474788],[18926205.431087866,2692596.21474788],[19037535.01626311,2692596.21474788],[19148846.482863177,2692596.21474788],[19260167.00875084,2692596.21474788],[19371487.53463849,2692596.21474788],[19482799.001238555,2692596.21474788],[19594128.58641381,2692596.21474788],[19705449.112301473,2692596.21474788],[19816769.638189115,2692596.21474788],[19921404.409836456,2692596.21474788],[20037472.002420496,2692596.21474788]
            ]]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        assert_json_eq(&out, geojson);

        // ne_10m_admin_0_country_points.fgb, first feature
        let geojson =
            r#"{"type": "Point", "coordinates": [2223639.4731508396,-15878634.348995442]}"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        assert_json_eq(&out, geojson);

        // geoz_lod1_gebaeude_max_3d_extract.fgb, first feature
        let geojson = r#"{
            "type": "MultiPolygon",
            "coordinates": [
                [
                    [
                        [2683312.339,1247968.33],[2683311.496,1247964.044],[2683307.761,1247964.745],[2683309.16,1247973.337],[2683313.003,1247972.616],[2683312.339,1247968.33]
                    ],[
                        [2683312.339,1247968.33],[2683313.003,1247972.616],[2683313.003,1247972.616],[2683312.339,1247968.33],[2683312.339,1247968.33]
                    ],[
                        [2683307.761,1247964.745],[2683311.496,1247964.044],[2683311.496,1247964.044],[2683307.761,1247964.745],[2683307.761,1247964.745]
                    ],[
                        [2683311.496,1247964.044],[2683312.339,1247968.33],[2683312.339,1247968.33],[2683311.496,1247964.044],[2683311.496,1247964.044]
                    ]
                ],[
                    [
                        [2683309.16,1247973.337],[2683307.761,1247964.745],[2683307.761,1247964.745],[2683309.16,1247973.337],[2683309.16,1247973.337]
                    ]
                ],[
                    [
                        [2683312.339,1247968.33],[2683311.496,1247964.044],[2683307.761,1247964.745],[2683309.16,1247973.337],[2683313.003,1247972.616],[2683312.339,1247968.33]
                    ],[
                        [2683313.003,1247972.616],[2683309.16,1247973.337],[2683309.16,1247973.337],[2683313.003,1247972.616],[2683313.003,1247972.616]
                    ]
                ]
            ]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        assert_json_eq(&out, geojson);

        Ok(())
    }

    #[test]
    fn geometries3d() -> Result<()> {
        let geojson = r#"{"type": "LineString", "coordinates": [[1,1,10],[2,2,20]]}"#;
        let mut out: Vec<u8> = Vec::new();
        let mut writer = GeoJsonWriter::new(&mut out);
        writer.dims = CoordDimensions::xyz();
        assert!(read_geojson(&mut geojson.as_bytes(), &mut writer).is_ok());
        assert_json_eq(&out, geojson);

        Ok(())
    }

    #[test]
    fn geometry_collection() -> Result<()> {
        let geojson = r#"{
            "type": "GeometryCollection",
            "geometries": [
                {
                    "type": "Point",
                    "coordinates": [100.1,0.1]
                },{
                    "type": "LineString",
                    "coordinates": [[101.1,0.1],[102.1,1.1]]
                }
            ]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        assert_json_eq(&out, geojson);
        Ok(())
    }

    #[test]
    fn feature_collection() -> Result<()> {
        // TODO: geojson reader does not have any handling of the "bbox" field, nor does it handle any unknown fields

        let geojson = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {
                        "id": "NZL",
                        "name": "New Zealand"
                    },
                    "geometry": {
                        "type": "MultiPolygon",
                        "coordinates": [[[
                            [173.020375,-40.919052],[173.247234,-41.331999],[173.958405,-40.926701],[174.247587,-41.349155],[174.248517,-41.770008],[173.876447,-42.233184],[173.22274,-42.970038],[172.711246,-43.372288],[173.080113,-43.853344],[172.308584,-43.865694],[171.452925,-44.242519],[171.185138,-44.897104],[170.616697,-45.908929],[169.831422,-46.355775],[169.332331,-46.641235],[168.411354,-46.619945],[167.763745,-46.290197],[166.676886,-46.219917],[166.509144,-45.852705],[167.046424,-45.110941],[168.303763,-44.123973],[168.949409,-43.935819],[169.667815,-43.555326],[170.52492,-43.031688],[171.12509,-42.512754],[171.569714,-41.767424],[171.948709,-41.514417],[172.097227,-40.956104],[172.79858,-40.493962],[173.020375,-40.919052]
                        ]],[[
                            [174.612009,-36.156397],[175.336616,-37.209098],[175.357596,-36.526194],[175.808887,-36.798942],[175.95849,-37.555382],[176.763195,-37.881253],[177.438813,-37.961248],[178.010354,-37.579825],[178.517094,-37.695373],[178.274731,-38.582813],[177.97046,-39.166343],[177.206993,-39.145776],[176.939981,-39.449736],[177.032946,-39.879943],[176.885824,-40.065978],[176.508017,-40.604808],[176.01244,-41.289624],[175.239567,-41.688308],[175.067898,-41.425895],[174.650973,-41.281821],[175.22763,-40.459236],[174.900157,-39.908933],[173.824047,-39.508854],[173.852262,-39.146602],[174.574802,-38.797683],[174.743474,-38.027808],[174.697017,-37.381129],[174.292028,-36.711092],[174.319004,-36.534824],[173.840997,-36.121981],[173.054171,-35.237125],[172.636005,-34.529107],[173.007042,-34.450662],[173.551298,-35.006183],[174.32939,-35.265496],[174.612009,-36.156397]
                        ]]]
                    }
                }
            ]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        assert_json_eq(&out, geojson);

        // same feature as above except added quotes to name and New Zealand
        let geojson = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {
                        "id": "NZL",
                        "\"name\"": "\"New Zealand\""
                    },
                    "geometry": {
                        "type": "MultiPolygon",
                        "coordinates": [[[
                            [173.020375,-40.919052],[173.247234,-41.331999],[173.958405,-40.926701],[174.247587,-41.349155],[174.248517,-41.770008],[173.876447,-42.233184],[173.22274,-42.970038],[172.711246,-43.372288],[173.080113,-43.853344],[172.308584,-43.865694],[171.452925,-44.242519],[171.185138,-44.897104],[170.616697,-45.908929],[169.831422,-46.355775],[169.332331,-46.641235],[168.411354,-46.619945],[167.763745,-46.290197],[166.676886,-46.219917],[166.509144,-45.852705],[167.046424,-45.110941],[168.303763,-44.123973],[168.949409,-43.935819],[169.667815,-43.555326],[170.52492,-43.031688],[171.12509,-42.512754],[171.569714,-41.767424],[171.948709,-41.514417],[172.097227,-40.956104],[172.79858,-40.493962],[173.020375,-40.919052]
                        ]],[[
                            [174.612009,-36.156397],[175.336616,-37.209098],[175.357596,-36.526194],[175.808887,-36.798942],[175.95849,-37.555382],[176.763195,-37.881253],[177.438813,-37.961248],[178.010354,-37.579825],[178.517094,-37.695373],[178.274731,-38.582813],[177.97046,-39.166343],[177.206993,-39.145776],[176.939981,-39.449736],[177.032946,-39.879943],[176.885824,-40.065978],[176.508017,-40.604808],[176.01244,-41.289624],[175.239567,-41.688308],[175.067898,-41.425895],[174.650973,-41.281821],[175.22763,-40.459236],[174.900157,-39.908933],[173.824047,-39.508854],[173.852262,-39.146602],[174.574802,-38.797683],[174.743474,-38.027808],[174.697017,-37.381129],[174.292028,-36.711092],[174.319004,-36.534824],[173.840997,-36.121981],[173.054171,-35.237125],[172.636005,-34.529107],[173.007042,-34.450662],[173.551298,-35.006183],[174.32939,-35.265496],[174.612009,-36.156397]
                        ]]]
                    }
                }
            ]
        }"#;
        let mut out: Vec<u8> = Vec::new();
        assert!(read_geojson(geojson.as_bytes(), &mut GeoJsonWriter::new(&mut out)).is_ok());
        assert_json_eq(&out, geojson);

        Ok(())
    }

    #[test]
    fn conversions() {
        let geom: geo_types::Geometry<f64> = geo_types::Point::new(10.0, 20.0).into();
        assert_eq!(
            &geom.to_json().unwrap(),
            r#"{"type": "Point", "coordinates": [10,20]}"#
        );
    }

    fn assert_json_eq(a: &[u8], b: &str) {
        let a = std::str::from_utf8(a).unwrap();
        let a: serde_json::Value = serde_json::from_str(a).unwrap();
        let b: serde_json::Value = serde_json::from_str(b).unwrap();
        assert_eq!(a, b);
    }
}
