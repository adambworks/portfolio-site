import axios from 'axios';

const API_URL = 'http://localhost:8080/api'; // Actix backend

export const fetchProjects = async () => {
  const response = await axios.get(`${API_URL}/list_projects`);
  return response.data;
};

export const fetchProjectBySlug = async (slug: string)  =>{
    const response = await axios.get(`${API_URL}/projects/${slug}`);
    return response.data;
}