use std::io::Read;
use std::io::Write;
fn main(){

    let mut file = std::fs::File::create("Convicted Ministers.txt").expect("Failed");
    let info = vec!["   NAME OF COMMISSIONER", "     GEOPOLITICAL ZONE", "     MINISTRY"];
    let name = vec!["   Aigbogun Alamba Daudu", "   Murtala Afeez Bendu", "   Okorocha Calistus Ogbonna", "   Adewalw Jimoh Akanbi", "   Osazuwa Faith Etiyeye"];
    let g_z = vec!["       South West", "         North East",   "   South South", "        South West", "       South East"];
    let min = vec!["         Internal Affairs", "         Justice", "       Defense", "        Power & Steel", "        Petroleum"];
    let i1 = info[0];
    let i2 = info[1];
    let i3 = info[2];
    let n1 = name[0];
    let n2 = name[1];
    let n3 = name[2];
    let n4 = name[3];
    let n5 = name[4];
    let g1 = g_z[0];
    let g2 = g_z[1];
    let g3 = g_z[2];
    let g4 = g_z[3];
    let g5 = g_z[4];
    let m1 = min[0];
    let m2 = min[1];
    let m3 = min[2];
    let m4 = min[3];
    let m5 = min[4];

    file.write_all(i1.as_bytes()).expect("FAILED");
    file.write_all(i2.as_bytes()).expect("FAILED");
    file.write_all(i3.as_bytes()).expect("FAILED");
    file.write_all("\n".as_bytes()).expect("FAILED");
    file.write_all(n1.as_bytes()).expect("FAILED");
    file.write_all(g1.as_bytes()).expect("FAILED");
    file.write_all(m1.as_bytes()).expect("FAILED");
    file.write_all("\n".as_bytes()).expect("FAILED");
    file.write_all(n2.as_bytes()).expect("FAILED");
    file.write_all(g2.as_bytes()).expect("FAILED");
    file.write_all(m2.as_bytes()).expect("FAILED");
    file.write_all("\n".as_bytes()).expect("FAILED");
    file.write_all(n3.as_bytes()).expect("FAILED");
    file.write_all(g3.as_bytes()).expect("FAILED");
    file.write_all(m3.as_bytes()).expect("FAILED");
    file.write_all("\n".as_bytes()).expect("FAILED");
    file.write_all(n4.as_bytes()).expect("FAILED");
    file.write_all(g4.as_bytes()).expect("FAILED");
    file.write_all(m4.as_bytes()).expect("FAILED");
    file.write_all("\n".as_bytes()).expect("FAILED");
    file.write_all(n5.as_bytes()).expect("FAILED");
    file.write_all(g5.as_bytes()).expect("FAILED");
    file.write_all(m5.as_bytes()).expect("FAILED");
        

        println!("DOC CREATED");
  let mut file = std::fs::File::open("Convicted Ministers.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  println!("{}", contents);
    

}
