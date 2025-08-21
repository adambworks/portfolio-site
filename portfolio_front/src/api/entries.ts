import api from './auth';
import type { Entry } from '../structs/entry';

export const fetchEntriesForChapter = async (ChapterId: number): Promise<Entry[]> => {
  const response = await api.get(`/chapters/${ChapterId}/entries`);
  return response.data;
};

