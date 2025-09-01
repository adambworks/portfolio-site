import api from './auth';
import type { Chapter, NewChapter } from '../structs/chapter';

export const fetchChapters = async (id: number): Promise<Chapter[]> => {
  const response = await api.get(`/projects/${id}/chapters`);
  return response.data;
};
export const fetchChapter = async (slug: string, index: number) => {
  const response = await api.get(`/projects/${slug}/chapter/${index}`);
  return response.data;
};

export const createChapter = async (chapter: NewChapter): Promise<Chapter> => {
  const response = await api.post('/admin/chapters', chapter);
  return response.data;
};

export const updateChapter = async (id: number, chapter: Chapter): Promise<Chapter> => {
  const response = await api.put(`/admin/chapters/${id}`, chapter);
  return response.data;
};

export const deleteChapter = async (id: number): Promise<void> => {
  await api.delete(`/admin/chapters/${id}`);
};