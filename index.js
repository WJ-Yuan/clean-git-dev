#!/usr/bin/env node
import { execSync } from 'node:child_process';
import process from 'node:process';
import readline from 'node:readline';

const main = () => {
  // welcome msg
  console.log(
    '==========================\nWelcome to git dev clean!\n==========================',
  );

  // question
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question(
    'please enter your exclude rules(main, master, develop is in default), use , to split the rules:\n',
    (rules) => {
      const ruleList = rules.split(',');
      ruleList.unshift('master', 'main', 'develop');
      console.log('the rules list:\n', ruleList);

      rl.question(
        'do you want to continue to delete dev git branch?(Enter y/n)',
        (result) => {
          if (result === 'y') {
            const branches = execSync('git branch').toString('utf-8');
            const washArray = (branches || '')
              .trim()
              .split('\n')
              .map((branch) => branch.trim())
              .filter(
                (branch) => !ruleList.some((rule) => branch.includes(rule)),
              );
            console.log('These branches are to be removed', washArray);

            rl.question('Do you ensure to remove them?(y/n)', (ensure) => {
              if (ensure === 'y') {
                washArray.forEach((branch) => {
                  execSync(`git branch -d ${branch}`);
                });

                const rest = execSync('git branch -l').toString('utf-8');
                console.log('The rest branches:\n', rest);
                rl.close();
              } else {
                process.exit(0);
              }
            });
          } else {
            process.exit(0);
          }
        },
      );
    },
  );
};

main();
