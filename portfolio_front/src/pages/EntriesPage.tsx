import { useEffect, useState } from "react";
import { Link, useParams } from "react-router";
import { fetchEntriesForChapter } from "../api/entries";
import type { Entry } from "../structs/entry";
import type { Chapter } from "../structs/chapter";
import { fetchChapter, fetchChapters } from "../api/chapters";
import HamburgerMenu from "../modules/global_buttons";
import type { Project } from "../structs/project";
import { fetchProjects } from "../api/projects";

function chapter_buttons(chapter: Chapter, slug: string, chapters: Chapter[]) {
	return (
		<div className="grid grid-cols-2 space-x-20">
			<div
				className={`rounded  bg-colorb px-4 py-2 text-colorc transition hover:bg-colorb/70 shadow-sm ${
					chapter.index <= 1 ||
					!chapters.some((c) => c.index === chapter.index - 1)
						? "invisible"
						: ""
				}`}
			>
				<Link to={`/projects/${slug}/chapter/${chapter.index - 1}`}>
					<h2>
						Previous :{" "}
						{chapters.find((c) => c.index === chapter.index - 1)?.name}
					</h2>
				</Link>
			</div>

			<div
				className={`rounded  bg-colorb px-4 py-2 text-colorc transition hover:bg-colorb/70 shadow-sm ${
					!chapters.some((c) => c.index === chapter.index + 1)
						? "invisible"
						: ""
				}`}
			>
				<Link to={`/projects/${slug}/chapter/${chapter.index + 1}`}>
					<h2>
						Next : {chapters.find((c) => c.index === chapter.index + 1)?.name}
					</h2>
				</Link>
			</div>
		</div>
	);
}

export default function EntriesPage() {
	const { slug, chapterIndex } = useParams();
	const [entries, setEntries] = useState<Entry[]>([]);
	const [chapter, setChapter] = useState<Chapter | null>(null);
	const [loading, setLoading] = useState(true);
	const [chapters, setChapters] = useState<Chapter[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const API_URL = import.meta.env.VITE_API_URL;

  useEffect(() => {
    fetchProjects().then(setProjects).catch(console.error);
  }, []);
	useEffect(() => {
		if (slug && chapterIndex) {
			fetchChapter(slug, +chapterIndex).then(setChapter).catch(console.error);
		}
	}, [slug, chapterIndex]);

	useEffect(() => {
		if (chapter == null) return;
		fetchEntriesForChapter(chapter.id)
			.then(setEntries)
			.catch(console.error)
			.finally(() => setLoading(false));
	}, [chapter]);

	useEffect(() => {
		if (chapter == null) return;
		fetchChapters(chapter.project_id).then(setChapters).catch(console.error);
	}, [chapter]);

	if (loading) {
		return <div className="p-4">
      {HamburgerMenu(projects)}
      Loading...</div>;
	}
	if (!chapter) return <div className="p-4">
    {HamburgerMenu(projects)}
     Invalid Chapter</div>;
	if (!slug) return <div className="p-4">
    {HamburgerMenu(projects)}
     Invalid Slug</div>;

	return (
		<div>
      {HamburgerMenu(projects)}
			<div className="mx-auto max-w-31/32 p-4">
				<h1 className="mb-4 text-2xl font-bold">{chapter?.name}</h1>
				{entries.length === 0 ? (
					<p className="text-gray-600">
						No entries available for this chapter.
					</p>
				) : (
					<div className="grid grid-cols-1 gap-4">
						{entries.map((entry) => (
							<div
								key={entry.id}
								className=" border-8 rounded-lg bg-colorc p-4"
							>
								<p className="text-2xl">{entry.text}</p>
								{entry.date != null && entry.date != "" && (
									<p className="text-colora">Date: {entry.date}</p>
								)}
								{entry.image && (
									<img
                    src={`${API_URL}/images/${entry.image}`}
										alt=""
										className="mb-2 place-self-center w-full h-auto object-contain"
									/>
								)}
							</div>
						))}
					</div>
				)}
			</div>
			{chapter_buttons(chapter, slug, chapters)}
		</div>
	);
}
