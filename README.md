# On Deck
*Future svg Image*

## Purpose
This project started-out as a portfolio project and exploration of what full-stack web Rust development would feel like for someone who was completely brand-new to it. I surely did attempt to escape tutorial-hell with this one. The web-game implements Rocket for handling server requests and managing the Redis database. The web-frontend subsists solely off yew along side some web_sys function calls. All assets see within the game were created from svg files as well as normal CSS animations.

**Disclaimer**: Given that this was made as a portfolio project, I did not enlist the aide of any AI or Content Generation machine. (I did at once try: further explained with the DOCUMENTATION.md).

## TODO:
- [ ] User game authentication
	- [X] Add additional hashed private_id_key
	- [X] Allow server the ability to authenticate user fire requests
		- [X] Client-side half of sealing-key process, 
			- [X] Find a way to have the client register a new challenge with the game_state requests
			- [X] Incorporate it in the JSON response sent towards the fire handle function.
	- [ ] Spectating feature for lost or spectator players
- [ ] Update board_page rendering
	- [X] Add an empty board style
	- [X] Add an ship board style
	- [ ] Add Ship Placement Prompt
		- [ ] Create a ready stage where everyone can place their ships
- [ ] Salvo based progression
	- [ ] New salvo round when all players have fired
	- [ ] End sequence (win / loss)
- [ ] Settings Panel
	- [ ] Flesh out
	- [ ] Allow Player Id Updating
	- [ ] Graphics Adjuster (Stars, Clouds, Blur, Animations)
- [ ] Game Assets (SVG)
	- [ ] Tab favicon image
	- [ ] Github image
	- [ ] In-game ship assets
		- [ ] Day
		- [ ] Night
	- [ ] Optional
		- [ ] Planes
		- [ ] Clouds
- [ ] Configure usability for smaller screens
- [ ] Additional Paper Logs
	- [ ] Documentation
	- [ ] Initialize Changelog
	- [ ] Update License
- [ ] Optimize Code

### Additional TODO:
- [ ] Determine how many players a single instance can support (How many Rocket workers are there?)
- [ ] Perhaps retry to incorporate Server-Side events into the application again
- [X] Investigate a better encryption algorithm that could be used to verify requests.
