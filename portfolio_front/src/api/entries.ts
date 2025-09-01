import api from './auth';
import type { Entry, NewEntry } from '../structs/entry';

export const fetchEntriesForChapter = async (ChapterId: number): Promise<Entry[]> => {
  const response = await api.get(`/chapters/${ChapterId}/entries`);
  return response.data;
};

export const createEntry = async (entry: NewEntry): Promise<Entry> => {
  const response = await api.post('/admin/entries', entry);
  return response.data;
};

export const updateEntry = async (id: number, entry: Entry): Promise<Entry> => {
  const response = await api.put(`/admin/entries/${id}`, entry);
  return response.data;
};

export const deleteEntry = async (id: number): Promise<void> => {
  await api.delete(`/admin/entries/${id}`);
};

