import { Link } from 'react-router';

export default function HomePage() {
  return (
    <div className="max-w-fit mx-auto p-24 text-white">
      <h1 className="text-4xl font-bold mb-4">Welcome to My Portfolio</h1>

      <section className="mb-8">
        <h2 className="text-2xl font-semibold mb-2">About This Site</h2>
        <p>
          This is a full-stack portfolio site built using Rust (Actix Web) for the backend and React (Vite) for the frontend.
          It showcases a collection of projects, each with detailed chapters and entries that walk through the development process.
          
          This site is both a technical showcase and a place where I document my work and ideas.
        </p>
      </section>

      <section className="mb-8">
        <h2 className="text-2xl font-semibold mb-2">About Me</h2>
        <p>
          I'm a computer scinces graduate from RIT who enjoys learning new things.

          
        </p>
      </section>

      <section className="mb-8">
        <Link
          to="/projects"
          className="inline-block bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition"
        >
          View Projects
        </Link>
      </section>

      <section className="mb-1">
        <h2 className="text-2xl font-semibold mb-2">Connect</h2>
        <a
          href="https://www.linkedin.com/in/adamhbock"
          target="_blank"
          rel="noopener noreferrer"
          className="text-blue-600 hover:underline"
        >
          LinkedIn Profile
        </a>
      </section>

      <section>
        <h2 className="text-2xl font-semibold mb-2">Contact</h2>
        <p>
          email me at:{' '}
          <a
            href="mailto:adam.bock@gmail.com"
            className="text-blue-600 hover:underline"
          >
            adam.bock@gmail.com
          </a>
        </p>
        text or call at:{' '}
        <a className='text-blue-600'>201-580-0667</a>
      </section>
    </div>
  );
}
