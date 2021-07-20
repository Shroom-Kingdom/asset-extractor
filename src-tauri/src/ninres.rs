use crate::Result;

use image::{DynamicImage, ImageBuffer};
use ninres::{Bfres, EmbeddedFile, NinRes, NinResFile, Sarc};
use std::{
    cmp, fs,
    path::{self, PathBuf},
};

pub fn extract_ninres(file: &NinResFile, mut path: PathBuf) -> Result<()> {
    path = PathBuf::from("/home/marior/ninres");
    if !path.exists() {
        fs::create_dir_all(path.clone())?;
    }
    match file {
        NinResFile::Bfres(bfres) => {
            extract_bfres(bfres, path.clone(), path)?;
        }
        NinResFile::Sarc(sarc) => {
            extract_sarc(sarc, path.clone(), path)?;
        }
    }
    Ok(())
}

fn extract_bfres(bfres: &Bfres, out_path: PathBuf, base_path: PathBuf) -> Result<()> {
    for file in bfres.get_embedded_files().iter() {
        match file {
            EmbeddedFile::BNTX(bntx) => {
                for texture in bntx.get_textures().iter() {
                    for (tex_count, mips) in texture.get_texture_data().iter().enumerate() {
                        if let Some(mip) = mips.iter().next() {
                            let width = cmp::max(1, texture.width);
                            let height = cmp::max(1, texture.height);
                            let buf = if let Some(image) =
                                ImageBuffer::from_raw(width, height, mip.clone())
                            {
                                image
                            } else {
                                continue;
                            };
                            let image = DynamicImage::ImageRgba8(buf);

                            let path_diff = out_path.strip_prefix(&base_path).unwrap();
                            let mut file_name = path_diff
                                .to_string_lossy()
                                .replace("output", "")
                                .replace(".Nin_NX_NVN", "")
                                .replace(path::MAIN_SEPARATOR, "_")
                                .replace("Model_", "");
                            file_name.push_str(&format!(
                                "{}_{}.png",
                                texture.get_name(),
                                tex_count
                            ));
                            let mut path = base_path.clone();
                            path.push(file_name);
                            if let Err(_err) = image.save(path) {
                                // skip
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn extract_sarc(sarc: &Sarc, out_path: PathBuf, base_path: PathBuf) -> Result<()> {
    sarc.get_sfat_nodes()
        .iter()
        .map(move |sfat| -> Result<_> {
            let mut path = out_path.clone();
            if let Some(sfat_path) = sfat.get_path() {
                path.push(sfat_path);

                let data = if let Some(data) = sfat.get_data_decompressed() {
                    data
                } else {
                    sfat.get_data()
                };

                if let Ok(file) = data.as_ninres() {
                    path.set_extension(file.get_extension().to_string());
                    match &file {
                        NinResFile::Bfres(bfres) => {
                            let mut path0 = path.clone();
                            path0.pop();
                            path0.push(path.file_stem().unwrap());
                            extract_bfres(bfres, path0, base_path.clone())?;
                        }
                        NinResFile::Sarc(sarc) => {
                            let mut path0 = path.clone();
                            path0.pop();
                            path0.push(path.file_stem().unwrap());
                            extract_sarc(sarc, path0, base_path.clone())?;
                        }
                    }
                }
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}
