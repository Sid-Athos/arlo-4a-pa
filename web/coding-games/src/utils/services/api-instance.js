import axios from "axios";

const instance = axios.create({
    baseURL: 'https://dev.mikusupremacy.fr/api',
    headers: {
        'Content-Type': 'application/json',
        'api-key' : 'coding_games',
        'Authorization':sessionStorage.getItem("token")
    },
});





const setAuthorizationHeader = (token) => {
    instance.interceptors.request.use(conf => {
        conf.headers.Authorization = token
        return conf
    })

}

export const ApiInstance = {
    getInstance: instance,
    updateAuthorizationHeader: setAuthorizationHeader
}