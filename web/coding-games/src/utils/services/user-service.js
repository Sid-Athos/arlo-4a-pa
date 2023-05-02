import {AxiosInstance} from "./axios-instance";

const signIn = async (userSignIn) => {
    return (await AxiosInstance.getInstance.post('/user/login', userSignIn));
}

const signUp = async (userSignUp) => {
    return await AxiosInstance.getInstance.post('/user/create', userSignUp);
}

export const UserService = {
    signIn: signIn,
    signUp: signUp
}