import Link from 'next/link'
import { FaGithub, FaLinkedin } from 'react-icons/fa'

export default function Footer() {
    return (
        <div className="flex flex-col items-center p-4 sm:flex-row sm:justify-between">
            <div className="flex space-x-2 m:mb-0">
                <Link href="https://github.com/OleMarGlo" target="_blank">
                    <FaGithub className="text-2xl" />
                </Link>
                <Link href="https://no.linkedin.com/in/ole-marius-glomsrud-0a677a354" target="_blank">
                    <FaLinkedin className="text-2xl" />
                </Link>
            </div>
            <p className="text-center">&copy; Ole Marius Glomsrud. All rights reserved</p>
        </div>
    );
}
