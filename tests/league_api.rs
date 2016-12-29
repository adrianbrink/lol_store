extern crate lol_store;

use lol_store::league_api::*;
use lol_store::models::Participant;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_summoner_ids() {
        let api_client = APIClient::new().unwrap();
        let participant_1 = Participant {
            bot: false,
            champion_id: 1,
            profile_icon_id: 1,
            spell_1_id: 1,
            spell_2_id: 1,
            summoner_name: "n3wk1d".to_string(),
            team_id: 1,
        };
        let participant_2 = Participant {
            bot: false,
            champion_id: 1,
            profile_icon_id: 1,
            spell_1_id: 1,
            spell_2_id: 1,
            summoner_name: "awacatization".to_string(),
            team_id: 1,
        };
        let participants = vec![participant_1, participant_2];
        let summoner_names = api_client.get_summoner_names(participants);
        assert_eq!("n3wk1d".to_string(), summoner_names[0]);
        assert_eq!("awacatization".to_string(), summoner_names[1]);
    }

    #[test]
    fn create_request_url_for_get_summoner_ids() {
        let api_client = APIClient::new().unwrap();
        let names = vec!["n3wk1d".to_string(), "awacatization".to_string()];
        let url: String = api_client.create_request_url_for_get_summoner_ids(names);
        assert!(url.starts_with("https://euw.api.pvp.net/api/lol/euw/v1.4/summoner/by-name/n3wk1d,\
                          awacatization?api_key="));
    }

    #[test]
    fn request_get_summoner_ids() {
        let api_client = APIClient::new().unwrap();
        let expected_result = "{\"awacatization\":{\"id\":58889103,\"name\":\"awacatization\",\
                               \"profileIconId\":749,\"summonerLevel\":30,\"revisionDate\":\
                               1483042076000},\"n3wk1d\":{\"id\":19861577,\"name\":\"N3wK1d\",\
                               \"profileIconId\":42,\"summonerLevel\":30,\"revisionDate\":\
                               1477839577000}}";
        let names = vec!["n3wk1d".to_string(), "awacatization".to_string()];
        let actual_result = api_client.request_get_summoner_ids(names);
        assert_eq!(actual_result, expected_result);
    }
}
