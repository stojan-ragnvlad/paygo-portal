FROM php:8.3.8-cli-alpine3.20

RUN wget https://raw.githubusercontent.com/composer/getcomposer.org/\
163a517dbb7eba0eb25633061c76e648392c6738/web/installer -O - -q | php -- --quiet

RUN php composer.phar require "twig/twig:^3.10.0"

