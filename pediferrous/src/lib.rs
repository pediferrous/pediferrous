use std::{fs::File, io::Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ObjRef {
    offs: usize,
    gen: usize,
}

pub fn gen_test_file() {
    let mut obj_refs: Vec<ObjRef> = Vec::new();
    let mut f = File::create("./test.pdf").expect("Could not create file in current directory.");

    let mut wrote = 0;

    wrote += f
        .write(b"%PDF-1.0\n")
        .expect("Could not write PDF header to the file.")
        + 1;

    wrote += write_catalog_dict(&mut f, &mut obj_refs, wrote);

    // let str_offs = wrote;

    obj_refs.push(ObjRef {
        offs: wrote,
        gen: 0,
    });

    // wrote += f
    //     .write(b"4 0 obj\n(Hello there)\nendobj\n")
    //     .expect("Could not write test content in pdf file.");

    let xref_offs = wrote;
    wrote += f.write(b"xref\n").unwrap(); // start cross-reference table

    // cross-ref subsection, start from obj 0, 1 object in subsection
    wrote += f
        .write(format!("0 {}\n", obj_refs.len()).as_bytes())
        .unwrap();

    for obj_ref in obj_refs.iter() {
        let ref_string = format!("{:010} {:05} n \n", obj_ref.offs, obj_ref.gen);
        wrote += f.write(ref_string.as_bytes()).unwrap();
    }

    wrote += f.write(b"trailer\n").unwrap();
    wrote += f.write(b"<<\n/Size 1\n").unwrap(); // one object in cross reference
    wrote += f.write(b"/Root 1 0 R\n>>\n").unwrap(); // Reference to catalog dictionary
    wrote += f.write(b"startxref\n").unwrap();
    wrote += f.write(xref_offs.to_string().as_bytes()).unwrap();
    wrote += f.write(b"\n").unwrap();

    wrote += f
        .write(b"%%EOF")
        .expect("Could not write EOF marker in pdf file.");

    println!("Wrote: {wrote}");
}

fn write_catalog_dict(mut f: impl Write, obj_refs: &mut Vec<ObjRef>, start_offs: usize) -> usize {
    let mut wrote = start_offs;

    obj_refs.push(ObjRef {
        offs: wrote,
        gen: 0,
    });

    wrote += f.write(b"1 0 obj\n").unwrap();
    wrote += f.write(b"<< ").unwrap();
    wrote += f.write(b"/Type /Catalog ").unwrap();
    wrote += f.write(b"/Pages 2 0 R ").unwrap();
    // wrote += f.write(b"/PageMode /UseThumbs\n").unwrap();

    wrote += f.write(b">>\nendobj\n").unwrap();

    wrote += write_page_tree(f, obj_refs, wrote);

    wrote
}

fn write_page_tree(mut f: impl Write, obj_refs: &mut Vec<ObjRef>, start_offs: usize) -> usize {
    let mut wrote = start_offs;

    obj_refs.push(ObjRef {
        offs: wrote,
        gen: 0,
    });

    wrote += f.write(b"2 0 obj\n").unwrap();
    wrote += f.write(b"<< /Type /Pages ").unwrap();
    wrote += f.write(b"/Kids [3 0 R] ").unwrap();
    wrote += f.write(b"/Count 1 >>\nendobj\n").unwrap();

    obj_refs.push(ObjRef {
        offs: wrote,
        gen: 0,
    });

    wrote += f.write(b"3 0 obj\n").unwrap();
    wrote += f.write(b"<< /Type /Page ").unwrap();
    wrote += f.write(b"/Parent 2 0 R ").unwrap();
    wrote += f.write(b"/MediaBox [0 0 200 200] ").unwrap();
    // wrote += f.write(b"/Contents 4 0 R\n").unwrap();
    wrote += f.write(b"/Resources << >> ").unwrap();
    wrote += f.write(b">>\nendobj\n").unwrap();

    wrote
}
