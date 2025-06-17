import { Link, useParams } from "react-router";
import { useEffect, useState } from "react";
import { fetchProjectBySlug, fetchProjects } from "../api/projects";
import type { Project } from "../structs/project";
import type { Chapter } from "../structs/chapter";
import { fetchChapters } from "../api/chapters";
import HamburgerMenu from "../modules/global_buttons";

function chapter_descriptor(project: Project, chapter_index: number) {
	if (project.chapter_descriptor != null) {
		return (
			<h2>
				{project.chapter_descriptor}: {chapter_index}
			</h2>
		);
	} else {
		return;
	}
}

export default function ProjectDetailPage() {
	const { slug } = useParams<{ slug: string }>();
	const [project, setProject] = useState<Project | null>(null);
	const [chapters, setChapters] = useState<Chapter[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);

  useEffect(() => {
    fetchProjects().then(setProjects).catch(console.error);
  }, []);
	useEffect(() => {
		if (slug) {
			fetchProjectBySlug(slug).then(setProject).catch(console.error);
		}
	}, [slug]);

	useEffect(() => {
		if (!project) return;
		fetchChapters(project.id).then(setChapters).catch(console.error);
	}, [project]);

	if (!project) return <div className="p-4">{HamburgerMenu(projects)} Invalid project</div>;

	const chapterList =
		chapters.length > 0 ? (
			chapters.map((chapter) => (
				<Link
					key={chapter.index}
					to={`/projects/${project.slug}/chapter/${chapter.index}`}
					className="block rounded-lg border-b-8 border-t-8 bg-white p-2 shadow transition hover:shadow-md"
				>
					{chapter_descriptor(project, chapter.index)}
					<h2 className="mb-1 text-lg font-semibold">{chapter.name}</h2>
				</Link>
			))
		) : (
			<p className="text-gray-500">No chapters available.</p>
		);

	return (
    
		<div className="p-4">
                  {HamburgerMenu(projects)}
      
			{project.image !== null && project.image != "" && (
				<img
					src={`http://localhost:8080/api/images/${project.image}`}
					alt=""
					className="mb-2 h-40 w-full rounded object-cover"
				/>
			)}
			<h1 className="mb-2 text-2xl font-bold">{project.name}</h1>
			<p className="mb-4 text-gray-500">{project.date_started}</p>
			<p>overview: {project.overview}</p>
			<div className="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3">
				{chapterList}
			</div>
		</div>
	);
}
