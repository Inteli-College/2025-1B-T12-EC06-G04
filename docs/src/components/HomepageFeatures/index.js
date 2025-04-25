import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'Aumente a produtividade em vistorias',
    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        Fácil e rápido e seguro, o 14 bits oferece suporte para sua vistoria em tempo real.
      </>
    ),
  },
  {
    title: 'Foco no que importa',
    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
       Nosso objetivo é incorporar tecnologias que tornem o processo de vistoria um trabalho mais simples, seguro e rápido. <br/> Chega de subir em Gôndolas, Plataformas elevatórias ou Andaimes!
      </>
    ),
  },
  {
    title: 'Classificação automática de fissuras',
    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
      O 14 bis conta com algorítmo de classificação integrado que define a gravidade estimada de fissuras estruturais.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
