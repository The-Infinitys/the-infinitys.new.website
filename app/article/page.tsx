import fs from 'fs';
import path from 'path';
import React from 'react';
import './page.css';

interface Article {
  date: string;
  id: string;
  thumbnail: string;
  title: string;
}

interface Month {
  [key: string]: Article[];
}

interface Year {
  [key: string]: Month;
}

interface ArticleData {
  [key: string]: Year;
}

const getArticleData = async (): Promise<ArticleData> => {
  const filePath = path.join(process.cwd(), 'library/article-loader/export/article-data.json');
  const jsonData = fs.readFileSync(filePath, 'utf8');
  const articles: ArticleData = JSON.parse(jsonData);
  return articles;
};

export default async function Page() {
  const articleData: ArticleData = await getArticleData();
  const years = Object.keys(articleData).sort((a, b) => parseInt(b) - parseInt(a));
  return (
    <>
      <section className='first'>
        {years.map((year) => {
          const months = Object.keys(articleData[year]).sort((a, b) => {
            const monthOrder = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
            return monthOrder.indexOf(a) - monthOrder.indexOf(b);
          });
          return (
            <div key={year}>
              <h2>{year}</h2>
              {months.map((month) => {
                return (
                  <div key={month}>
                    <h3>{month}</h3>
                    {articleData[year][month].map((article: Article) => (
                      <div key={article.id}>
                        <p>{article.title}</p>
                      </div>
                    ))}
                  </div>
                );
              })}
            </div>
          );
        })}
      </section>
    </>
  )
}
