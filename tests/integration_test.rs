use anyhow::Result;
use rusty_battleship::{
    game::Game,
    game_rules::GameRules,
    game_stage::GameStage,
    point::{Orientation, Point},
    ship::Ship,
};

#[test]
fn it_plays_start_to_finish() -> Result<()> {
    let mut game = Game::new(GameRules {
        submarine_limit: 1,
        destroyer_limit: 1,
        cruiser_limit: 0,
        battleship_limit: 0,
    });

    assert_eq!(game.stage, GameStage::Waiting);

    let (player_a, _) = game.connect()?;
    let (player_b, _) = game.connect()?;

    assert_eq!(game.stage, GameStage::PlayerShips(None));

    game.place_figure(player_a, Ship::Submarine, Point::new(1, 0)?)?;
    game.remove_figure(player_a, Point::new(1, 0)?)?;

    game.place_figure(player_a, Ship::Submarine, Point::new(0, 0)?)?;
    game.place_figure(
        player_a,
        Ship::Destroyer(Orientation::Horizontal),
        Point::new(2, 2)?,
    )?;

    assert_eq!(game.stage, GameStage::PlayerShips(Some(player_b)));

    game.place_figure(player_b, Ship::Submarine, Point::new(0, 0)?)?;
    game.place_figure(
        player_b,
        Ship::Destroyer(Orientation::Horizontal),
        Point::new(2, 2)?,
    )?;

    assert_eq!(game.stage, GameStage::PlayerShoots(player_a));

    game.shoot(player_a, Point::new(1, 1)?)?;

    assert_eq!(game.stage, GameStage::PlayerShoots(player_b));

    game.shoot(player_b, Point::new(0, 0)?)?;

    assert_eq!(game.stage, GameStage::PlayerShoots(player_b));

    game.shoot(player_b, Point::new(2, 2)?)?;
    game.shoot(player_b, Point::new(3, 2)?)?;

    assert_eq!(game.stage, GameStage::PlayerWins(player_b));

    return Ok(());
}
