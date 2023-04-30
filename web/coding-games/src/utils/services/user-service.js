import {AxiosInstance} from "./axios-instance";

const signIn = async (userSignIn) => {
    await AxiosInstance.getInstance.post('/user/login', userSignIn);
}

const signUp = async (userSignUp) => {
    await AxiosInstance.getInstance.post('/user/create', userSignUp);
}

export const UserService = {
    signIn: signIn,
    signUp: signUp
}