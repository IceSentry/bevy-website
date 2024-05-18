{% set data = load_data(path=path) %}
## Contributors

A huge thanks to the {{ data.contributors | length }} contributors that made this release (and associated docs) possible! In random order:

<ul class="contributors">
{% for contributor in data.contributors %}
<li class="contributor__name">{{ contributor.name }}</li>
{% endfor %}
</ul>
