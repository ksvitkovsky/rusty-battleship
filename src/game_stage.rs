use anyhow::Result;

type PlayerId = u8;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GameStage {
    Waiting = 1,
    PlayerShips(Option<PlayerId>) = 2,
    PlayerShoots(PlayerId) = 3,
    PlayerWins(PlayerId) = 4,
}

impl GameStage {
    pub fn try_into_u8(&self, my_id: u8) -> Result<u8> {
        return match self {
            GameStage::Waiting => Ok(1),
            GameStage::PlayerShips(player) => {
                if player.is_some() && player.unwrap() == my_id {
                    return Ok(2);
                } else {
                    return Ok(3);
                }
            }
            GameStage::PlayerShoots(player_id) => {
                if *player_id == my_id {
                    return Ok(4);
                } else {
                    return Ok(5);
                }
            }
            GameStage::PlayerWins(player_id) => {
                if *player_id == my_id {
                    return Ok(6);
                } else {
                    return Ok(7);
                }
            }
        };
    }
}
