import api from './auth';
import type { Project, NewProject } from '../structs/project';

export const fetchProjects = async (): Promise<Project[]> => {
  const response = await api.get('/list_projects');
  return response.data;
};

export const fetchProjectBySlug = async (slug: string) => {
  const response = await api.get(`/projects/${slug}`);
  return response.data;
};

export const createProject = async (project: NewProject): Promise<Project> => {
  const response = await api.post('/admin/projects', project);
  return response.data;
};

export const updateProject = async (id: number, project: Project): Promise<Project> => {
  const response = await api.put(`/admin/projects/${id}`, project);
  return response.data;
};

export const deleteProject = async (id: number): Promise<void> => {
  await api.delete(`/admin/projects/${id}`);
};