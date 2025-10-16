# BdfWriter for Nastran Mesh Export
This Rust module provides a simple utility to export mesh data into a Nastran-compatible BDF (Bulk Data File) format. It supports writing vertex and face data as `GRID` and `CTRIA3` cards respectively.

## Features

- Writes structured BDF headers for SOL 101 analysis
- Supports multiple meshes with offset tracking
- Outputs vertex coordinates with fixed-width formatting
- Generates triangular elements (`CTRIA3`) from face indices

## Usage
```rust
let mesh = Mesh::new(...); // Your mesh data
let mut writer = BdfWriter::new("output.bdf")?;
writer.run(&[mesh])?;
```

## 전체 소스
### BdfWriter
```rust
use std::fs::File;
use std::io::{Result, Write};
use crate::mesh::mesh::Mesh;

pub struct BdfWriter {
    path: String,
}

impl BdfWriter {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            path: path.to_string(),
        })
    }
    pub fn run(&mut self, meshes: &[Mesh]) -> Result<()> {
        let mut file = File::create(&self.path)?;
        let mut node_offset = 1;
        let mut elem_offset = 1;
        writeln!(file, "ASSIGN UNIT=12, FORM=FORMATTED")?;
        writeln!(file, "SOL 101")?;
        writeln!(file, "CEND")?;
        writeln!(file, "SEALL = ALL")?;
        writeln!(file, "SUPER = ALL")?;
        writeln!(file, "TITLE = BDF file")?;
        writeln!(file, "ECHO = NONE")?;
        writeln!(file, "SUBCASE 1")?;
        writeln!(file, "SUBTITLE=Geometry Data")?;
        writeln!(file, "DISPLACEMENT(SORT1,REAL)=ALL")?;
        writeln!(file, "SPCFORCES(SORT1,REAL)=ALL")?;
        writeln!(file, "STRESS(SORT1,REAL,VONMISES,BILIN)=ALL")?;
        writeln!(file, "BEGIN BULK")?;
        writeln!(file, "PARAM   POST    -1")?;
        writeln!(file, "PARAM   AUTOSPC YES")?;
        writeln!(file, "PARAM   PRTMAXIM YES")?;

        for mesh in meshes {
            // Write GRID cards
            for (i, pt) in mesh.vertices.iter().enumerate() {
                let id = node_offset + i as u32;
                writeln!(
                    file,
                    "{:<8}{:>8}{:>8}{:>8}{:>8}{:>8}",
                    "GRID",
                    id,
                    "", // CP field (blank)
                    format_bdf_field(pt.x),
                    format_bdf_field(pt.y),
                    format_bdf_field(pt.z)
                )?;
            }

            // Write CTRIA3 cards
            for (i, face) in mesh.faces.iter().enumerate() {
                let id = elem_offset + i as u32;
                let v1 = node_offset + face[0];
                let v2 = node_offset + face[1];
                let v3 = node_offset + face[2];
                writeln!(
                    file,
                    "{:<8}{:>8}{:>8}{:>8}{:>8}{:>8}",
                    "CTRIA3",
                    id,
                    1, // PID
                    v1,
                    v2,
                    v3
                )?;
            }
            node_offset += mesh.vertices.len() as u32;
            elem_offset += mesh.faces.len() as u32;
        }
        writeln!(file, "ENDDATA")?;
        Ok(())
    }
}

/// Formats the coordinate field to a fixed width of 8 columns.
fn format_bdf_field(val: f64) -> String {
    let s = format!("{:.6}", val); //Six digits after the decimal point.
    let trimmed = if s.len() > 8 {
        &s[..8] //Prevents overlap or intrusion.

    } else {
        s.as_str()
    };
    format!("{:>8}", trimmed) //Right-align the content.
}

```

---
