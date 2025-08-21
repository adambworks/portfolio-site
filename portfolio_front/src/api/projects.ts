import api from './auth';
import type { Project } from '../structs/project';

export const fetchProjects = async (): Promise<Project[]> => {
  const response = await api.get('/list_projects');
  return response.data;
};

export const fetchProjectBySlug = async (slug: string) => {
  const response = await api.get(`/projects/${slug}`);
  return response.data;
};