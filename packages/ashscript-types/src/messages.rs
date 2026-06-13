use hexx::Hex;

use crate::{actions::ActionsByKind, chunk::Chunk, keyframe::KeyFrame, player::PlayerId};

pub enum SocketMessage {
    KeyFrame(KeyFrame), // The entire game state
    Actions(ActionsByKind), // All actions that happened last tick
    GetNewChunkFrame(Hex), // requesting data for a chunk
    GetUpdateChunkFrame(Hex), // requestion actions for a specified chunk
    NewChunkFrame(Chunk), // chunk data for a newly-viewed chunk
    UpdateChunkFrame(ActionsByKind), // chunk data for an already-viewed chunk
    NewChunkMapFrame(Vec<(Hex, PlayerId)>), // positions and player id of objects in a chunk
}