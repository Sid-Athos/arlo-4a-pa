use std::ptr::null;
use bcrypt::{hash, verify};
use axum::http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use crate::database::database_error::{database_error_invalid_input, DatabaseError};
use crate::database::repository::ranking_repository::RankingRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::ranking::Ranking;

pub struct RankingService {
    pub ranking_repository: RankingRepository
}

impl RankingService {
    pub fn new(ranking_repository: RankingRepository) -> Self {
        RankingService {
            ranking_repository
        }
    }

    pub async fn init_ranking(&self, user_id: i32, game_id: i32) -> Result<Ranking, StatusCode> {
        self.ranking_repository.create_ranking(game_id,user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_ranking_by_friends(&self, user_id: i32, game_id: i32) -> Result<Vec<Ranking>, StatusCode> {
        self.ranking_repository.get_ranking_by_friends(user_id,game_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_ranking_by_user(&self, user_id: i32) -> Result<Vec<Ranking>, StatusCode> {
        self.ranking_repository.get_ranking_by_user(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_ranking_by_game(&self, game_id: i32) -> Result<Vec<Ranking>, StatusCode> {
        self.ranking_repository.get_ranking_by_game(game_id).await.map_err(database_error_to_status_code)
    }

    pub async fn delete_ranking_by_user(&self, user_id: i32) -> Result<Vec<Ranking>, StatusCode> {
        self.ranking_repository.delete_rankings_by_user_id(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn delete_ranking_by_game(&self, game_id: i32) -> Result<Vec<Ranking>, StatusCode> {
        self.ranking_repository.delete_rankings_by_game_id(game_id).await.map_err(database_error_to_status_code)
    }

    /*pub async fn update_ranking(&self,  game_id: i32, player_1_id : i32, player_2_id : i32, tie : bool, winner_id : i32) -> Result<Vec<Ranking>, StatusCode> {
        let mut game_id = game_id;
        let mut player_1_id = player_1_id;
        let mut player_2_id = player_2_id;
        let winner = self.ranking_repository.get_one_ranking(game_id , player_1_id).await.map_err(database_error_to_status_code);
        let loser = self.ranking_repository.get_one_ranking(game_id, player_2_id).await.map_err(database_error_to_status_code);


        let elo_average = winner.clone().unwrap().rank - loser.clone().unwrap().rank;
        let power = (10 as f64).powf(elo_average as f64 / 400 as f64);
        let win_chance = (1.0 / (1.0 + power) );
        let winner_k = if winner.clone().unwrap().nb_games < 10 {50.0} else {20.0};
        let loser_k = if winner.clone().unwrap().nb_games < 10 {50.0} else {20.0};
        let winner_new_ranking;
        let loser_new_ranking;
        if tie == true {
            winner_new_ranking = winner.unwrap().rank as f64 + (winner_k * (0.5 - win_chance));
            loser_new_ranking = loser.unwrap().rank as f64 + (loser_k * (0.5 - (win_chance + 1.0)));
        }else{
            winner_new_ranking = winner.unwrap().rank as f64 + (winner_k * (1.0 - win_chance));
            loser_new_ranking = loser.unwrap().rank as f64 + (loser_k * (0.0 - (1.0 - win_chance)));
        }
        println!("elo_average : {} // power : {} // win_chance : {} // winner_k : {} // loser_k : {} // winner_new_ranking : {} // loser_new_ranking : {}",elo_average,power,win_chance,winner_k,loser_k,winner_new_ranking,loser_new_ranking);
        let mut result = Vec::new();
        if winner.clone().unwrap().rank > loser.clone().unwrap().rank {
            result.push(self.ranking_repository.update_ranking(game_id, winner_id, winner_new_ranking.round() as i32).await.map_err(database_error_to_status_code).unwrap());
            result.push(self.ranking_repository.update_ranking(game_id,loser_id,loser_new_ranking.round() as i32).await.map_err(database_error_to_status_code).unwrap());

        }

        Ok(result)

    }*/

    /*pub async fn update_rankings_multi(&self,  game_id: i32, players_id : Vec<i32>, tie : bool, winner_id : Vec<i32>) -> Result<Vec<Ranking>, StatusCode> {

    }*/

    pub async fn update_ranking2(&self,  game_id: i32, player_1_id : i32, player_2_id : i32, tie : bool, winner_id : i32) -> Result<Vec<Ranking>, StatusCode> {
        let p1 = self.ranking_repository.get_one_ranking(game_id , player_1_id).await.map_err(database_error_to_status_code);
        let p2 = self.ranking_repository.get_one_ranking(game_id, player_2_id).await.map_err(database_error_to_status_code);


        let elo_diff = p1.clone().unwrap().rank - p2.clone().unwrap().rank;
        let power = (10 as f64).powf(elo_diff as f64 / 400 as f64);
        let win_chance = 1.0 - (1.0 / (1.0 + power) );
        let p1_k = if p1.clone().unwrap().nb_games < 10 {100.0} else {50.0};
        let p2_k = if p2.clone().unwrap().nb_games < 10 {100.0} else {50.0};
        let p1_new_ranking;
        let p2_new_ranking;
        let ratio = if p1.clone().unwrap().rank > p2.clone().unwrap().rank {(p1.clone().unwrap().rank / p2.clone().unwrap().rank) as f64} else {(p2.clone().unwrap().rank / p1.clone().unwrap().rank) as f64};
        if tie == true {
            p1_new_ranking = p1.unwrap().rank as f64 + ((p1_k * (0.5 - win_chance)) * ratio);
            p2_new_ranking = p2.unwrap().rank as f64 + ((p2_k * (0.5 - (1.0 - win_chance))) * ratio);
        }else if player_1_id == winner_id {
            p1_new_ranking = p1.unwrap().rank as f64 + ((p1_k * (1.0 - win_chance)) * ratio);
            p2_new_ranking = p2.unwrap().rank as f64 + ((p2_k * (0.0 - (1.0 - win_chance))) * ratio);
        }else {
            p1_new_ranking = p1.unwrap().rank as f64 + ((p1_k * (0.0 - win_chance)) * ratio);
            p2_new_ranking = p2.unwrap().rank as f64 + ((p2_k * (1.0 - (1.0 - win_chance))) * ratio);
        }
        println!("elo_diff : {} // power : {} // win_chance : {} // winner_k : {} // loser_k : {} // winner_new_ranking : {} // loser_new_ranking : {}", elo_diff, power, win_chance, p1_k, p2_k, p1_new_ranking, p2_new_ranking);
        let mut result = Vec::new();
        result.push(self.ranking_repository.update_ranking(game_id, player_1_id, p1_new_ranking.round() as i32).await.map_err(database_error_to_status_code).unwrap());
        result.push(self.ranking_repository.update_ranking(game_id,player_2_id,p2_new_ranking.round() as i32).await.map_err(database_error_to_status_code).unwrap());

        Ok(result)
    }

    }