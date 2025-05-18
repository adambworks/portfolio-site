import axios from 'axios';
import type { Entry } from '../structs/entry';

const API_URL = import.meta.env.VITE_API_URL; // Actix backend

export const fetchEntriesForChapter = async (ChapterId:number): Promise<Entry[]> => {
  const response = await axios.get(`${API_URL}/chapters/${ChapterId}/entries`);
  return response.data;
};

