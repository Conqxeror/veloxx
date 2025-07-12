import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Lightning Fast Performance',
    Svg: require('@site/static/img/undraw_speed.svg').default,
    description: (
      <>
        Built in Rust for maximum performance. Outperforms pandas by 10x and 
        competes with Polars while maintaining a minimal footprint.
      </>
    ),
  },
  {
    title: 'Multi-Language Support',
    Svg: require('@site/static/img/undraw_programming.svg').default,
    description: (
      <>
        Native Rust library with seamless Python and JavaScript bindings. 
        Use the same powerful API across your entire tech stack.
      </>
    ),
  },
  {
    title: 'Zero Dependencies',
    Svg: require('@site/static/img/undraw_lightweight.svg').default,
    description: (
      <>
        Extremely lightweight with minimal external dependencies. 
        Perfect for resource-constrained environments and edge computing.
      </>
    ),
  },
  {
    title: 'Memory Efficient',
    Svg: require('@site/static/img/undraw_memory.svg').default,
    description: (
      <>
        Optimized memory usage with efficient data structures and 
        zero-copy operations wherever possible.
      </>
    ),
  },
  {
    title: 'Type Safe',
    Svg: require('@site/static/img/undraw_safety.svg').default,
    description: (
      <>
        Leverages Rust's type system for compile-time guarantees and 
        memory safety without garbage collection overhead.
      </>
    ),
  },
  {
    title: 'Rich Analytics',
    Svg: require('@site/static/img/undraw_analytics.svg').default,
    description: (
      <>
        Comprehensive data processing capabilities including filtering, 
        grouping, aggregations, joins, and statistical operations.
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
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

export default function HomepageFeatures(): JSX.Element {
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