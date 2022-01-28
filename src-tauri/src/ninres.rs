use crate::Result;

use image::{
    png::{CompressionType, FilterType, PngEncoder},
    DynamicImage, GenericImageView, ImageBuffer,
};
use ninres::{Bfres, EmbeddedFile, NinRes, NinResFile, Sarc};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    cmp,
    io::Cursor,
    path::{self, PathBuf},
    sync::RwLock,
};

pub fn bundle_ninres(
    file: &NinResFile,
    builder: &RwLock<tar::Builder<Cursor<Vec<u8>>>>,
    path: PathBuf,
    mtime: u64,
) -> Result<()> {
    match file {
        NinResFile::Bfres(bfres) => {
            extract_bfres(bfres, builder, path.clone(), path, mtime)?;
        }
        NinResFile::Sarc(sarc) => {
            extract_sarc(sarc, builder, path.clone(), path, mtime)?;
        }
    }
    Ok(())
}

fn extract_bfres(
    bfres: &Bfres,
    builder: &RwLock<tar::Builder<Cursor<Vec<u8>>>>,
    out_path: PathBuf,
    base_path: PathBuf,
    mtime: u64,
) -> Result<()> {
    for file in bfres.get_embedded_files().iter() {
        match file {
            EmbeddedFile::BNTX(bntx) => {
                for texture in bntx.get_textures().iter() {
                    let texture_name = texture.get_name();
                    if texture_name.starts_with("WU_") {
                        continue;
                    }
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

                            if texture_name.contains("_Field_")
                                && !texture_name.contains("_Field_anime_")
                            {
                                (0..48u32)
                                    .into_par_iter()
                                    .map(|y| -> Result<()> {
                                        for x in 0..16 {
                                            let image = image.crop_imm(x * 16, y * 16, 16, 16);

                                            let transparency_bytes: Vec<_> = image
                                                .as_bytes()
                                                .iter()
                                                .skip(3)
                                                .step_by(4)
                                                .copied()
                                                .collect();
                                            let transparent_bytes =
                                                vec![0; transparency_bytes.len()];
                                            if transparency_bytes == transparent_bytes {
                                                continue;
                                            }

                                            let mut file_name = file_name.clone();
                                            file_name.push_str(&format!(
                                                "{}_{}_{}.png",
                                                texture.get_name(),
                                                tex_count,
                                                x + 16 * y
                                            ));

                                            let mut image_data = vec![];
                                            let encoder = PngEncoder::new_with_quality(
                                                &mut image_data,
                                                CompressionType::Best,
                                                FilterType::NoFilter,
                                            );
                                            encoder.encode(
                                                image.as_bytes(),
                                                image.width(),
                                                image.height(),
                                                image.color(),
                                            )?;

                                            let mut header = tar::Header::new_gnu();
                                            header.set_size(image_data.len() as u64);
                                            header.set_mode(0o644);
                                            header.set_mtime(mtime);
                                            header.set_cksum();
                                            builder.write().unwrap().append_data(
                                                &mut header,
                                                file_name.clone(),
                                                &image_data[..],
                                            )?;
                                        }
                                        Ok(())
                                    })
                                    .collect::<Result<()>>()?;
                            }
                            file_name.push_str(&format!(
                                "{}_{}.png",
                                texture.get_name(),
                                tex_count
                            ));

                            let mut image_data = vec![];
                            let encoder = PngEncoder::new_with_quality(
                                &mut image_data,
                                CompressionType::Best,
                                FilterType::NoFilter,
                            );
                            encoder.encode(
                                image.as_bytes(),
                                image.width(),
                                image.height(),
                                image.color(),
                            )?;

                            let mut header = tar::Header::new_gnu();
                            header.set_size(image_data.len() as u64);
                            header.set_mode(0o644);
                            header.set_mtime(mtime);
                            header.set_cksum();
                            builder.write().unwrap().append_data(
                                &mut header,
                                file_name,
                                &image_data[..],
                            )?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn extract_sarc(
    sarc: &Sarc,
    builder: &RwLock<tar::Builder<Cursor<Vec<u8>>>>,
    out_path: PathBuf,
    base_path: PathBuf,
    mtime: u64,
) -> Result<()> {
    sarc.get_sfat_nodes()
        .into_par_iter()
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
                            extract_bfres(bfres, builder, path0, base_path.clone(), mtime)?;
                        }
                        NinResFile::Sarc(sarc) => {
                            let mut path0 = path.clone();
                            path0.pop();
                            path0.push(path.file_stem().unwrap());
                            extract_sarc(sarc, builder, path0, base_path.clone(), mtime)?;
                        }
                    }
                }
            }
            Ok(())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}
