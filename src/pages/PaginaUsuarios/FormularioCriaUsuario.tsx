import { invoke } from "@tauri-apps/api";
import { useContext, useState } from "react";
import { Link } from "react-router-dom";
import { FeedbackContext } from "../../routes/appRouter";
import { validaNome } from "../../interfaces/ZodInputs";

export default function FormularioCriaUsuario() {

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [usernameErr, setInputUsernameErr] = useState('');
    const [passwordErr, setInputPasswordErr] = useState('');
    const [buttonDisabled, setButtonDisabled] = useState(false);
    const { createFeedback, manageLoading } = useContext(FeedbackContext);


    const criaUsuario = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();


        let validar_nome = validaNome(username);
        let validar_senha = validaNome(password);

        if (validar_nome.success && validar_senha.success) {
            manageLoading(true);
            setButtonDisabled(true);
            try {
                await invoke("create_user", { username, password });
                window.location.href = '/usuarios';
                createFeedback(false, "Usuário criado.")
            } catch (e) {
                createFeedback(true, String(e))
            } finally {
                manageLoading(false);
            }
        } else {
            if (!validar_nome.success) {
                setInputUsernameErr("Nome deve conter, ao menos, 3 caracteres.");
            }
            if (!validar_senha.success) {
                setInputPasswordErr("Senha deve conter, ao menos, 3 caracteres.");
            }

        }


    }

    return (
        <div className=" h-full flex flex-col items-center justify-center">
            <h1 className=" text-3xl mb-4">Criar novo usuário</h1>
            <form onSubmit={e => criaUsuario(e)} className="flex flex-col w-96">
                <label className=" border-none text-xs font-semibold text-slate-400" htmlFor="username">NOME</label>
                <input onChange={e => setUsername(e.target.value)} className="mb-2 shadow appearance-none border rounded w-full py-2 px-3 text-slate-700 leading-tight focus:outline-none focus:shadow-outline" type="text" name="" id="username" />
                <span className=" mb-2 text-xs text-red-500">{usernameErr}</span>
                <label className="text-xs font-semibold text-slate-400" htmlFor="">SENHA</label>
                <input onChange={e => setPassword(e.target.value)} className="shadow appearance-none border rounded w-full py-2 px-3 text-slate-700 leading-tight focus:outline-none focus:shadow-outline" type="text" name="" id="password" />
                <span className=" mb-2 text-xs text-red-500">{passwordErr}</span>
                <div className=" mt-4 flex items-center w-full justify-between">
                    <Link to={'/usuarios'}><p className=" text-slate-400 underline cursor-pointer ml-2">Voltar</p></Link>
                    {!buttonDisabled && <button type="submit" className=" text-xl w-36 transition-all hover:bg-transparent hover:text-emerald-300 border border-emerald-300  bg-emerald-300 text-emerald-700 font-semibold p-2 rounded">Confirmar</button>}
                    {buttonDisabled && <button disabled className=" opacity-50 text-xl w-36 transition-all hover:bg-transparent hover:text-emerald-300 border border-emerald-300  bg-emerald-300 text-emerald-700 font-semibold p-2 rounded">Confirmar</button>}
                </div>
            </form>
        </div>
    )
}