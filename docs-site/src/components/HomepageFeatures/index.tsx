import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';
import { 
  Zap, 
  Code, 
  Feather, 
  MemoryStick, 
  Shield, 
  BarChart3 
} from 'lucide-react';

type FeatureItem = {
  title: string;
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Lightning Fast Performance',
    Icon: Zap,
    description: (
      <>
        Built in Rust for maximum performance. Outperforms pandas by 10x and 
        competes with Polars while maintaining a minimal footprint.
      </>
    ),
  },
  {
    title: 'Multi-Language Support',
    Icon: Code,
    description: (
      <>
        Native Rust library with seamless Python and JavaScript bindings. 
        Use the same powerful API across your entire tech stack.
      </>
    ),
  },
  {
    title: 'Zero Dependencies',
    Icon: Feather,
    description: (
      <>
        Extremely lightweight with minimal external dependencies. 
        Perfect for resource-constrained environments and edge computing.
      </>
    ),
  },
  {
    title: 'Memory Efficient',
    Icon: MemoryStick,
    description: (
      <>
        Optimized memory usage with efficient data structures and 
        zero-copy operations wherever possible.
      </>
    ),
  },
  {
    title: 'Type Safe',
    Icon: Shield,
    description: (
      <>
        Leverages Rust's type system for compile-time guarantees and 
        memory safety without garbage collection overhead.
      </>
    ),
  },
  {
    title: 'Rich Analytics',
    Icon: BarChart3,
    description: (
      <>
        Comprehensive data processing capabilities including filtering, 
        grouping, aggregations, joins, and statistical operations.
      </>
    ),
  },
];

function Feature({title, Icon, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Icon className={clsx(styles.featureIcon, 'feature-icon')} size={64} />
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