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
				className={`rounded border border-gray-900 bg-white p-4 shadow-sm ${
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
				className={`rounded border border-gray-900 bg-white p-4 shadow-sm ${
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

export default function ChapterPage() {
	const { slug, chapterIndex } = useParams();
	const [entries, setEntries] = useState<Entry[]>([]);
	const [chapter, setChapter] = useState<Chapter | null>(null);
	const [loading, setLoading] = useState(true);
	const [chapters, setChapters] = useState<Chapter[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);

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
			<div className="mx-auto max-w-7xl p-8">
				<h1 className="mb-4 text-2xl font-bold">{chapter?.name}</h1>
				{entries.length === 0 ? (
					<p className="text-gray-600">
						No entries available for this chapter.
					</p>
				) : (
					<div className="grid grid-cols-1 gap-0">
						{entries.map((entry) => (
							<div
								key={entry.id}
								className=" border-l-4 border-r-4 bg-white p-4 shadow-sm"
							>
								<p className="text-gray-700">{entry.text}</p>
								{entry.date != null && entry.date != "" && (
									<p className="text-gray-700">Date: {entry.date}</p>
								)}
								{entry.image && (
									<img
										src={`http://localhost:8080/api/images/${entry.image}`}
										alt=""
										className="mb-2 h-60 w-full rounded object-cover"
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
