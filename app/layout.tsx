"use client";

import Link from "next/link";
import Image from "next/image";
import "./globals.css";
import { useEffect, useState } from "react";

// Import Images
import infinite_icon from "../public/infinite-icon.svg";
import infinite_logo_black from "../public/infinite-logo-black.svg";
import infinite_logo_white from "../public/infinite-logo-white.svg";
import navigation_open_icon from "../public/layout/header/navigation/open.svg";
import navigation_close_icon from "../public/layout/header/navigation/close.svg";

function NavigationMenus({ isOpen }: { isOpen: boolean }) {
  const data = [
    {
      name: "Home",
      link: "/",
    },
    {
      name: "Article",
      link: "/article",
    },
    {
      name: "Contact",
      link: "/contact",
    },
  ];
  return (
    <nav id="header-navigation" style={{ right: isOpen ? "0" : "calc(-1 * var(--navigation-width))" }} >
      <ul>
        {data.map((value, index) => (
          <li key={index} >
            <Link href={value.link}>{value.name}</Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    const headerNavigationButton = document.querySelector("#header-navigation-button") as HTMLElement | null;
    if (headerNavigationButton) {
      headerNavigationButton.onclick = () => {
        setIsOpen(prevState => !prevState);
      };
    }
  }, []);

  return (
    <html lang="ja">
      <body>
        <header>
          <Link href="/">
            <Image id="header-infinite-icon" src={infinite_icon} alt="" />
          </Link>
          <Image src={infinite_logo_white} alt="THE INFINITY'S" className="dark-mode header-infinite-logo" />
          <Image src={infinite_logo_black} alt="THE INFINITY'S" className="light-mode header-infinite-logo" />
          <button id="header-navigation-button">
            <div className="navigation-icons">
              <Image src={navigation_open_icon} alt="Open Navigation" className={`icon open ${isOpen ? 'hidden' : 'visible'}`} />
              <Image src={navigation_close_icon} alt="Close Navigation" className={`icon close ${isOpen ? 'visible' : 'hidden'}`} />
            </div>
          </button>
          <NavigationMenus isOpen={isOpen} />
        </header>
        <main>
          {children}
        </main>
        <footer></footer>
      </body>
    </html>
  );
}
