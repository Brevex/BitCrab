mod domain;
mod usecases;
mod adapters;

use adapters::controllers::torrent_controller::handle_torrent;

fn main() -> Result<(), domain::errors::TorrentError>
{
    let file_path = "./src/sample.torrent";
    handle_torrent(file_path)
}
