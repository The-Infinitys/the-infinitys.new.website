import Image from "next/image";
import type { Metadata } from "next";
import "./page.css";

// Import Images
import infinite_icon from "../public/infinite-icon.svg";
import infinite_logo_white from "../public/infinite-logo-white.svg";
import infinite_logo_black from "../public/infinite-logo-black.svg";

export const metadata: Metadata = {
  title: "The Infinity's",
  description: "Created by Next.js",
};

export default function Home() {
  return (
    <>
      <section className="first">
      <Image className="inf-icon" src={infinite_icon} alt="" />
      <Image className="dark-mode inf-logo" src={infinite_logo_white} alt="" />
      <Image className="light-mode inf-logo" src={infinite_logo_black} alt="" />
      </section>
      <section className="about">
        <h2>About Us</h2>
        <p>We are a team of passionate developers and designers.</p>
      </section>
    </>
  );
}
