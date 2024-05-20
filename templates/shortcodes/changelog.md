{% set data = load_data(path=path) %}

## Full Changelog

The changes mentioned above are only the most appealing, highest impact changes that we've made this cycle.
Innumerable bug fixes, documentation changes and API usability tweaks made it in too.
For a complete list of changes, check out the PRs listed below.

{% for area in data.areas %}
### {{ area.name | join(sep=" + ") }}

<ul>
{% for pr in area.prs %}
<li><a href="https://github.com/bevyengine/bevy/pull/{{pr.number}}">{{pr.title | escape}}</a></li>
{% endfor %}
</ul>
{% endfor %}
