# On Deck
*Future svg Image*

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
- [ ] Salvo based progression
	- [ ] New salvo round when all players have fired
	- [ ] End sequence (win / loss)
- [ ] Settings Page
	- [ ] Style Page for main_page
	- [ ] User Player Id Updating
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

### Additional TODO:
- [ ] Determine how many players a single instance can support (How many Rocket workers are there?)
- [ ] Perhaps retry to incorporate Server-Side events into the application again
- [ ] Investigate a better encryption algorithm that could be used to verify requests.
