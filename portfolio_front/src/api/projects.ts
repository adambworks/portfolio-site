import axios from 'axios';
import type { Project } from '../structs/project';

const API_URL = import.meta.env.VITE_API_URL; // Actix backend
export const fetchProjects = async (): Promise<Project[]> => {
  const response = await axios.get(`${API_URL}/list_projects`);
  return response.data;
};

export const fetchProjectBySlug = async (slug: string)  =>{
    const response = await axios.get(`${API_URL}/projects/${slug}`);
    return response.data;
}