import api from './auth';
import type { Chapter } from '../structs/chapter';

export const fetchChapters = async (id: number): Promise<Chapter[]> => {
  const response = await api.get(`/projects/${id}/chapters`);
  return response.data;
};
export const fetchChapter = async (slug: string, index: number) => {
  const response = await api.get(`/projects/${slug}/chapter/${index}`);
  return response.data;
};