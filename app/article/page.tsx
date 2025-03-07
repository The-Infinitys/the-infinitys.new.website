import fs from "fs";
import path from "path";
import React from "react";
import Image from "next/image";
import Link from "next/link";
import "./page.css";

type Article = {
  date: string;
  id: string;
  thumbnail: string;
  title: string;
};

type Month = [] | Article[];

type Year = {
  [key: string]: Month;
};

type ArticleData = {
  [key: string]: Year;
};

const getArticleData = async (): Promise<ArticleData> => {
  const filePath = path.join(
    process.cwd(),
    "library/article-loader/export/article-data.json"
  );
  const jsonData = fs.readFileSync(filePath, "utf8");
  const articles: ArticleData = JSON.parse(jsonData);
  return articles;
};

export default async function Page() {
  const articleData: ArticleData = await getArticleData();
  const years = Object.keys(articleData).sort(
    (a, b) => parseInt(b) - parseInt(a)
  );
  return (
    <>
      <section className="first">
        {years.map((year) => {
          const months = Object.keys(articleData[year]).sort((a, b) => {
            const monthOrder = [
              "January",
              "February",
              "March",
              "April",
              "May",
              "June",
              "July",
              "August",
              "September",
              "October",
              "November",
              "December",
            ];
            return monthOrder.indexOf(a) - monthOrder.indexOf(b);
          });
          return (
            <div key={year}>
              <h2>{year}</h2>
              {months.map((month) => {
                return (
                  <div key={month}>
                    <h3>{month}</h3>
                    {(function () {
                      if (!articleData[year][month]) {
                        return null;
                      }
                      return articleData[year][month].map(
                        (article: Article) => {
                          const article_path: string = `https://the-infinitys.f5.si/article-${year}/${month}/${article.id}/`;
                          const thumb_path: string =
                            article_path + article.thumbnail;
                          return (
                            <div key={article.id}>
                              <Link href={article_path}>
                                <img src={thumb_path} alt={article.title} width={400} height={300} />
                                <h4>{article.title}</h4>
                                <p>{article.date}</p>
                              </Link>
                            </div>
                          );
                        }
                      );
                    })()}
                  </div>
                );
              })}
            </div>
          );
        })}
      </section>
    </>
  );
}
