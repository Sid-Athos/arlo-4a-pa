import axios from "axios";

const instance = axios.create({
    baseURL: 'https://dev.mikusupremacy/gamemanager/',
    headers: {
        'Content-Type': 'application/json',
        'api-key' : 'coding_games'
    },
});

const setAuthorizationHeader = (token) => {
    instance.interceptors.request.use(conf => {
        conf.headers.Authorization = token
        return conf
    })
}

export const GameManagerInstance = {
    getInstance: instance,
    updateAuthorizationHeader: setAuthorizationHeader
}