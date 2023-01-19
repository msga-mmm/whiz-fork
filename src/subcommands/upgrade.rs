use self_update::cargo_crate_version;

pub fn upgrade() -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("msga-mmm")
        .repo_name("whiz-fork")
        .bin_name("whiz")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
