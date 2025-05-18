import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL;

export const fetchChapters = async (id: number)  =>{
    const response = await axios.get(`${API_URL}/projects/${id}/chapters`);
    return response.data;
}
export const fetchChapter = async (slug: string,index: number) =>{
    const response = await axios.get(`${API_URL}/projects/${slug}/chapter/${index}`);
    return response.data;
}